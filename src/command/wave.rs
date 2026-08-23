use crate::command::Context;
use crate::command::Parser;
use crate::wave::Wave as WaveUtil;

/// WaveCmd — displays the complex wave representation of a sentence.
///
/// Usage: `wave "some text"`
///
/// Shows the real and imaginary parts, amplitude, and phase of the
/// sentence's superposition wave.
pub struct WaveCmd;

impl WaveCmd {
    /// Computes and prints the wave representation of the given text.
    pub fn apply(&self, ctx: &mut Context) -> bool {
        if ctx.arg.is_empty() {
            println!("  Usage: wave \"some text\"");
            return true;
        }

        let text = Parser::strip_quotes(ctx.arg);
        let w = WaveUtil::text(ctx.manifold, &text);

        println!("  Sentence wave:  ({:.6}, {:.6})", w.re, w.im);
        println!("  Amplitude:      {:.6}", w.norm());
        println!("  Phase:          {:.6} rad", w.arg());
        true
    }
}
