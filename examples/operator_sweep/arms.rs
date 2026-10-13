#[allow(unused_imports)]
use super::*;
use phistd::{Adam, Schedule};
use phical::{Fno, FnoConfig};

pub(crate) const QUARTERS: usize = 4;

/// One configuration under test; arms differ from the shipped one in one
/// thing each.
pub(crate) struct Arm {
    pub name: &'static str,
    pub slug: &'static str,
    pub cfg: FnoConfig,
    pub softmax: bool,
    pub adamw: bool,
    pub steps_per_epoch: usize,
}

impl Arm {
    pub fn objective(&self) -> &'static str {
        if self.softmax { "cross-entropy over the table" } else { "hinge vs one decoy" }
    }

    pub fn optimiser(&self) -> &'static str {
        if self.adamw { "AdamW 1e-3, warm-up + cosine" } else { "Adam" }
    }

    /// A fresh operator and optimiser for this arm (seed 7, as `bin/austen`).
    pub fn fresh(&self, epochs: usize) -> (Fno, Adam) {
        let fno = Fno::new(self.cfg, 7);
        let adam = if self.adamw {
            Adam::new(self.cfg.lr).with_decay(1e-3).with_schedule(Schedule::Cosine { warmup: self.steps_per_epoch, total: self.steps_per_epoch * epochs, floor: 0.05 })
        } else {
            Adam::new(self.cfg.lr)
        };
        (fno, adam)
    }
}

pub(crate) fn arms(data: &Data, _epochs: usize) -> Vec<Arm> {
    let space = data.space.space();
    let small = FnoConfig { space, width: 8, layers: 2, modes: 12, margin: 0.2, lr: 2e-3, tie: phical::Tie::Circle };
    let big = FnoConfig { space, width: 16, layers: 3, modes: 16, margin: 0.2, lr: 2e-3, tie: phical::Tie::Circle };
    // The 64 channels read as spherical harmonics of degree < 8: eight
    // degrees, all kept, one complex weight per degree (phical::Tie::Sphere).
    let sphere = FnoConfig { space, width: 8, layers: 2, modes: 8, margin: 0.2, lr: 2e-3, tie: phical::Tie::Sphere };
    let aniso = FnoConfig { space, width: 8, layers: 2, modes: 8, margin: 0.2, lr: 2e-3, tie: phical::Tie::Anisotropic };
    // The circle's winning shape (width 16, 3 layers, softmax) with the
    // anisotropic tie in place of the FFT.
    let aniso_big = FnoConfig { space, width: 16, layers: 3, modes: 8, margin: 0.2, lr: 2e-3, tie: phical::Tie::Anisotropic };
    let steps_per_epoch = data.train.len().div_ceil(64);
    vec![
        Arm { name: "hinge (shipped)", slug: "hinge", cfg: small, softmax: false, adamw: false, steps_per_epoch },
        Arm { name: "hinge, sphere tie (one weight per degree)", slug: "hinge_sphere", cfg: sphere, softmax: false, adamw: false, steps_per_epoch },
        Arm { name: "softmax β=8, sphere tie", slug: "softmax_sphere", cfg: sphere, softmax: true, adamw: false, steps_per_epoch },
        Arm { name: "hinge, anisotropic tie (one weight per l,|m|)", slug: "hinge_aniso", cfg: aniso, softmax: false, adamw: false, steps_per_epoch },
        Arm { name: "softmax, wide+deep, anisotropic tie", slug: "softmax_big_aniso", cfg: aniso_big, softmax: true, adamw: false, steps_per_epoch },
        Arm { name: "softmax β=8", slug: "softmax", cfg: small, softmax: true, adamw: false, steps_per_epoch },
        Arm { name: "softmax, wide+deep", slug: "softmax_big", cfg: big, softmax: true, adamw: false, steps_per_epoch },
        Arm { name: "softmax, wide+deep, AdamW+cosine", slug: "softmax_big_adamw", cfg: big, softmax: true, adamw: true, steps_per_epoch },
    ]
}
