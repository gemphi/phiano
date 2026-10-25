//! The primitives: Russell's algebra, the stories' sets, the personas'
//! three moves, and the little that a Lisp needs besides.

use super::{Machine, Value};
use crate::sys::set::{self as s, Class, Relation};
use crate::sys::{Agent, Row};

pub const NAMES: [&str; 47] = [
    "union", "meet", "difference", "compose", "converse", "closure", "ancestral", "power", "domain", "range", "field", "image",
    "relata", "referents", "restrict-domain", "restrict-range", "product", "identity", "included?", "transitive?", "symmetric?",
    "reflexive?", "count", "pairs", "class", "members", "next", "says", "scopes", "held", "ask", "tell", "told", "beings", "list", "judge",
    "car", "cdr", "cons", "length", "=", "<", "+", "-", "*", "print", "str",
];

fn pairs(v: &Value) -> Result<Relation, String> {
    match v {
        Value::Relation(p) => Ok(p.clone()),
        Value::List(items) => items
            .iter()
            .map(|it| match it {
                Value::List(ab) if ab.len() == 2 => Ok((ab[0].text().unwrap_or_default(), ab[1].text().unwrap_or_default())),
                other => Err(format!("not a pair: {other}")),
            })
            .collect(),
        Value::Nil => Ok(Relation::new()),
        other => Err(format!("not pairs: {other}")),
    }
}

fn class(v: &Value) -> Result<Class, String> {
    match v {
        Value::Class(c) => Ok(c.clone()),
        Value::List(items) => Ok(items.iter().filter_map(Value::text).collect()),
        Value::Nil => Ok(Class::new()),
        other => other.text().map(|t| [t].into_iter().collect()).ok_or_else(|| format!("not a class: {other}")),
    }
}

fn num(v: &Value) -> Result<f64, String> {
    match v {
        Value::Num(n) => Ok(*n),
        other => Err(format!("not a number: {other}")),
    }
}

fn arg<'a>(args: &'a [Value], i: usize, name: &str) -> Result<&'a Value, String> {
    args.get(i).ok_or_else(|| format!("{name}: argument {} missing", i + 1))
}

fn being<'a>(m: &'a mut Machine, v: &Value, name: &str) -> Result<&'a mut crate::sys::Being, String> {
    let who = v.text().ok_or_else(|| format!("{name}: who?"))?.to_lowercase().replace(' ', "-");
    let key = m.beings.keys().find(|k| **k == who || k.split('-').any(|w| w == who)).cloned().ok_or_else(|| format!("{name}: no persona {who}"))?;
    Ok(m.beings.get_mut(&key).unwrap())
}

pub fn call(m: &mut Machine, name: &'static str, a: Vec<Value>) -> Result<Value, String> {
    let p = |i: usize| pairs(arg(&a, i, name)?);
    let c = |i: usize| class(arg(&a, i, name)?);
    let t = |i: usize| arg(&a, i, name)?.text().ok_or_else(|| format!("{name}: a name expected"));
    Ok(match name {
        "union" => Value::Relation(s::union(&p(0)?, &p(1)?)),
        "meet" => Value::Relation(s::meet(&p(0)?, &p(1)?)),
        "difference" => Value::Relation(s::difference(&p(0)?, &p(1)?)),
        "compose" => Value::Relation(s::compose(&p(0)?, &p(1)?)),
        "converse" => Value::Relation(s::converse(&p(0)?)),
        "closure" => Value::Relation(s::closure(&p(0)?)),
        "ancestral" => Value::Relation(s::ancestral(&p(0)?)),
        "power" => Value::Relation(s::power(&p(0)?, num(arg(&a, 1, name)?)? as usize)),
        "domain" => Value::Class(s::domain(&p(0)?)),
        "range" => Value::Class(s::range(&p(0)?)),
        "field" => Value::Class(s::field(&p(0)?)),
        "image" => Value::Class(s::image(&p(0)?, &c(1)?)),
        "relata" => Value::Class(s::relata(&p(0)?, &t(1)?)),
        "referents" => Value::Class(s::referents(&p(0)?, &t(1)?)),
        "restrict-domain" => Value::Relation(s::restrict_domain(&p(0)?, &c(1)?)),
        "restrict-range" => Value::Relation(s::restrict_range(&p(0)?, &c(1)?)),
        "product" => Value::Relation(s::product(&c(0)?, &c(1)?)),
        "identity" => Value::Relation(s::identity(&c(0)?)),
        "included?" => Value::Bool(s::included(&p(0)?, &p(1)?)),
        "transitive?" => Value::Bool(s::is_transitive(&p(0)?)),
        "symmetric?" => Value::Bool(s::is_symmetric(&p(0)?)),
        "reflexive?" => Value::Bool(s::is_reflexive(&p(0)?)),
        "count" => Value::Num(match arg(&a, 0, name)? {
            Value::Relation(x) => x.len(),
            Value::Class(x) => x.len(),
            Value::List(x) => x.len(),
            _ => 0,
        } as f64),
        "pairs" => Value::Relation(pairs(&Value::List(a))?),
        "class" => Value::Class(class(&Value::List(a))?),
        "members" => Value::List(c(0)?.into_iter().map(Value::Str).collect()),
        "next" => {
            let path = t(0)?;
            Value::Relation(m.scopes.get(&path).map(|sc| sc.pairs()).ok_or_else(|| format!("no scope {path}"))?)
        }
        "says" => {
            let path = t(0)?;
            Value::Class(m.scopes.get(&path).map(|sc| sc.says.keys().cloned().collect()).ok_or_else(|| format!("no scope {path}"))?)
        }
        "scopes" => Value::List(m.scopes.keys().map(|k| Value::Str(k.clone())).collect()),
        "held" => {
            let kind = t(1)?;
            Value::Relation(s::held(being(m, arg(&a, 0, name)?, name)?, &kind))
        }
        "ask" => {
            let (word, kind) = (t(1)?, t(2)?);
            Value::List(being(m, arg(&a, 0, name)?, name)?.ask(&word, &kind).into_iter().map(|(o, w)| Value::List(vec![Value::Str(o), Value::Num(w)])).collect())
        }
        "tell" => {
            let (rel, kind) = (p(1)?, t(2)?);
            let b = being(m, arg(&a, 0, name)?, name)?;
            let rows: Vec<Row> = rel.iter().map(|(x, y)| Row::new(x, &kind, y)).collect();
            b.tell(&rows);
            Value::Num(rows.len() as f64)
        }
        "told" => Value::Num(being(m, arg(&a, 0, name)?, name)?.told() as f64),
        // Execution is the judge (docs/169 §4, docs/178): `(judge "src")`
        // compiles and runs BCPL on phicpl's machine; the verdict comes
        // back as rows — (name kind other) — for any being to be told.
        "judge" => {
            let v = phicpl::judge(&t(0)?, 1_000_000);
            Value::List(
                v.rows("judged")
                    .into_iter()
                    .map(|r| Value::List(vec![Value::Str(r.about), Value::Str(r.kind), Value::Str(r.other)]))
                    .collect(),
            )
        }
        "beings" => Value::List(m.beings.keys().map(|k| Value::Sym(k.clone())).collect()),
        "list" => Value::List(a),
        "car" => match arg(&a, 0, name)? {
            Value::List(v) => v[0].clone(),
            _ => Value::Nil,
        },
        "cdr" => match arg(&a, 0, name)? {
            Value::List(v) if v.len() > 1 => Value::List(v[1..].to_vec()),
            _ => Value::Nil,
        },
        "cons" => match arg(&a, 1, name)? {
            Value::List(v) => Value::List(std::iter::once(a[0].clone()).chain(v.iter().cloned()).collect()),
            _ => Value::List(vec![a[0].clone()]),
        },
        "length" => Value::Num(match arg(&a, 0, name)? {
            Value::List(v) => v.len(),
            _ => 0,
        } as f64),
        "=" => Value::Bool(a.windows(2).all(|w| w[0] == w[1])),
        "<" => Value::Bool(num(arg(&a, 0, name)?)? < num(arg(&a, 1, name)?)?),
        "+" => Value::Num(a.iter().map(num).sum::<Result<f64, _>>()?),
        "*" => Value::Num(a.iter().map(num).product::<Result<f64, _>>()?),
        "-" => Value::Num(num(arg(&a, 0, name)?)? - a[1..].iter().map(num).sum::<Result<f64, _>>()?),
        "print" => {
            println!("{}", a.iter().map(|v| v.to_string()).collect::<Vec<_>>().join(" "));
            Value::Nil
        }
        "str" => Value::Str(a.iter().filter_map(Value::text).collect::<Vec<_>>().join("")),
        other => return Err(format!("unknown primitive {other}")),
    })
}
