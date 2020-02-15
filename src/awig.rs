use crate::{Memory, Register};
use pretty::RcDoc;
use std::collections::HashSet;
use std::fs::File;
use std::io::prelude::*;
use std::path::Path;

#[derive(Clone, Debug)]
enum LastLayer {
    AWIG(String, String, String, Vec<Register>, Vec<Memory>),
}

trait ToDoc {
    fn to_doc(&self) -> RcDoc<()>;

    fn to_pretty_with_width(&self, width: usize) -> String {
        let mut w = Vec::new();
        self.to_doc().render(width, &mut w).unwrap();
        String::from_utf8(w).unwrap()
    }

    fn to_pretty(&self) -> String {
        self.to_pretty_with_width(100)
    }
}

fn round_width(width: u32) -> u32 {
    let base = 32;
    if width % base == 0 {
        width
    } else {
        ((width / base) + 1) * base
    }
}

fn max_sel(width: u32) -> u32 {
    let base = 32;
    if width % base == 0 {
        width / base
    } else {
        (width / base) + 1
    }
}

fn func_write_name(path: &str) -> String {
    format!("{}_write", path.replace(".", "_"))
}

fn func_read_name(path: &str) -> String {
    format!("{}_read", path.replace(".", "_"))
}

fn func_expr_eq<'a>(var: &str, val: u32) -> RcDoc<'a> {
    RcDoc::as_string(var)
        .append(RcDoc::space())
        .append(RcDoc::text("=="))
        .append(RcDoc::space())
        .append(RcDoc::as_string(val))
}

fn func_expr_lt<'a>(var: &str, val: u32) -> RcDoc<'a> {
    RcDoc::as_string(var)
        .append(RcDoc::space())
        .append(RcDoc::text("<"))
        .append(RcDoc::space())
        .append(RcDoc::as_string(val))
}

fn func_paren<'a>(expr: RcDoc<'a>) -> RcDoc<'a> {
    RcDoc::text("(").append(expr).append(RcDoc::text(")"))
}

fn func_bracket<'a>(expr: RcDoc<'a>) -> RcDoc<'a> {
    RcDoc::text("[").append(expr).append(RcDoc::text("]"))
}

fn func_begin_end<'a>(body: RcDoc<'a>) -> RcDoc<'a> {
    RcDoc::text("begin")
        .append(RcDoc::hardline())
        .append(body)
        .nest(2)
        .group()
        .append(RcDoc::hardline())
        .append(RcDoc::text("end"))
}

fn func_module<'a>(name: &str, body: RcDoc<'a>) -> RcDoc<'a> {
    RcDoc::text("module")
        .append(RcDoc::space())
        .append(RcDoc::as_string(name))
        .append(RcDoc::text(";"))
        .append(RcDoc::hardline())
        .append(body)
        .nest(2)
        .group()
        .append(RcDoc::hardline())
        .append(RcDoc::text("endmodule"))
}

fn func_ifeq<'a>(name: &str, hid: u32, body: RcDoc<'a>) -> RcDoc<'a> {
    RcDoc::text("if")
        .append(RcDoc::space())
        .append(func_paren(func_expr_eq(name, hid)))
        .append(RcDoc::hardline())
        .append(func_begin_end(body))
        .append(RcDoc::hardline())
}

fn func_else<'a>(body: RcDoc<'a>) -> RcDoc<'a> {
    RcDoc::text("else").append(RcDoc::hardline()).append(body)
}

fn func_arg(name: &str) -> RcDoc<()> {
    RcDoc::text("input")
        .append(RcDoc::space())
        .append(RcDoc::text("int"))
        .append(RcDoc::space())
        .append(RcDoc::as_string(name))
        .append(RcDoc::text(";"))
}

fn func_var_type<'a>(width: u32) -> RcDoc<'a> {
    let ty = RcDoc::as_string(width)
        .append(RcDoc::text("-"))
        .append(RcDoc::as_string(1))
        .append(RcDoc::text(":"))
        .append(RcDoc::as_string(0));
    func_bracket(ty)
}

fn func_var(name: &str, width: u32) -> RcDoc<()> {
    RcDoc::text("reg")
        .append(RcDoc::space())
        .append(func_var_type(width))
        .append(RcDoc::space())
        .append(RcDoc::as_string(name))
        .append(RcDoc::text(";"))
}

fn func_index<'a>(lhs: RcDoc<'a>, rhs: RcDoc<'a>) -> RcDoc<'a> {
    func_bracket(RcDoc::concat(vec![lhs, RcDoc::text("+:"), rhs]))
}

fn func_index_var<'a>(name: &str) -> RcDoc<'a> {
    let base = 32;
    let lhs = RcDoc::as_string(name)
        .append(RcDoc::text("*"))
        .append(RcDoc::as_string(base.clone()));
    let rhs = RcDoc::as_string(base.clone());
    func_bracket(RcDoc::concat(vec![lhs, RcDoc::text("+:"), rhs]))
}

fn func_assign<'a>(lhs: RcDoc<'a>, rhs: RcDoc<'a>) -> RcDoc<'a> {
    let fmt = RcDoc::space()
        .append(RcDoc::text("="))
        .append(RcDoc::space())
        .append(rhs)
        .append(RcDoc::text(";"));
    RcDoc::concat(vec![lhs, fmt])
}

fn func_return<'a>(value: RcDoc<'a>) -> RcDoc<'a> {
    RcDoc::text("return")
        .append(RcDoc::space())
        .append(value)
        .append(RcDoc::text(";"))
}

fn func_eval<'a>(value: RcDoc<'a>) -> RcDoc<'a> {
    RcDoc::nil().append(value).append(RcDoc::text(";"))
}

fn func_str<'a>(value: &str) -> RcDoc<'a> {
    RcDoc::text("\"")
        .append(RcDoc::as_string(value))
        .append(RcDoc::text("\""))
}

fn func_error<'a>(msg: &str) -> RcDoc<'a> {
    RcDoc::text("$error")
        .append(func_paren(func_str(msg)))
        .append(RcDoc::text(";"))
}

fn func_assert<'a>(expr: RcDoc<'a>, error_msg: &str) -> RcDoc<'a> {
    RcDoc::text("assert")
        .append(RcDoc::space())
        .append(func_paren(expr))
        .append(RcDoc::space())
        .append(RcDoc::text("else"))
        .append(RcDoc::space())
        .append(func_error(error_msg))
}

fn func_assert_lt<'a>(var: &str, val: u32) -> RcDoc<'a> {
    func_assert(func_expr_lt(var, val), &format!("{} out of bounds", var))
}

fn func_body<'a>(name: &str, rtype: &str, body: RcDoc<'a>) -> RcDoc<'a> {
    RcDoc::text("function")
        .append(RcDoc::space())
        .append(RcDoc::as_string(rtype))
        .append(RcDoc::space())
        .append(RcDoc::as_string(name))
        .append(RcDoc::text(";"))
        .append(RcDoc::hardline())
        .append(body)
        .nest(2)
        .group()
        .append(RcDoc::hardline())
        .append(RcDoc::text("endfunction"))
}

fn func_<'a>(name: &str, rtype: &str, prologue: RcDoc<'a>, stmt: RcDoc<'a>) -> RcDoc<'a> {
    let body = RcDoc::concat(vec![prologue, RcDoc::hardline(), func_begin_end(stmt)]);
    func_body(name, rtype, body)
}

fn func_read<'a>(name: &str, prologue: RcDoc<'a>, body: RcDoc<'a>) -> RcDoc<'a> {
    let path = func_read_name(name);
    func_(&path, "int", prologue, body)
}

fn func_write<'a>(name: &str, prologue: RcDoc<'a>, body: RcDoc<'a>) -> RcDoc<'a> {
    let path = func_write_name(name);
    func_(&path, "void", prologue, body)
}

fn func_read_register<'a>(path: &str, width: u32) -> RcDoc<'a> {
    let var = "data";
    let sel = "sel";
    let mut pvec = Vec::new();
    let mut bvec = Vec::new();
    let rval = RcDoc::concat(vec![RcDoc::as_string(&var), func_index_var(&sel)]);
    let round_index = func_index(
        RcDoc::as_string(0),
        RcDoc::as_string(round_width(width.clone())),
    );
    let index = func_index(RcDoc::as_string(0), RcDoc::as_string(width.clone()));
    let a = RcDoc::concat(vec![RcDoc::as_string(&var), round_index]);
    let b = RcDoc::concat(vec![RcDoc::as_string(&var), index]);
    pvec.push(func_arg(&sel));
    pvec.push(func_var(&var, round_width(width.clone())));
    bvec.push(func_assert_lt(&sel, max_sel(width.clone())));
    bvec.push(func_assign(a, RcDoc::as_string(0)));
    bvec.push(func_assign(b, RcDoc::as_string(path)));
    bvec.push(func_return(rval));
    let prologue = RcDoc::intersperse(pvec.into_iter(), RcDoc::hardline());
    let body = RcDoc::intersperse(bvec.into_iter(), RcDoc::hardline());
    func_read(path, prologue, body)
}

fn func_write_register<'a>(path: &str, width: u32) -> RcDoc<'a> {
    let var = "data";
    let sel = "sel";
    let val = "value";
    let mut pvec = Vec::new();
    let mut bvec = Vec::new();
    let round_index = func_index(
        RcDoc::as_string(0),
        RcDoc::as_string(round_width(width.clone())),
    );
    let index = func_index(RcDoc::as_string(0), RcDoc::as_string(width.clone()));
    let a = RcDoc::concat(vec![RcDoc::as_string(&var), round_index]);
    let b = RcDoc::concat(vec![RcDoc::as_string(&var), index]);
    let c = RcDoc::concat(vec![RcDoc::as_string(&var), func_index_var(&sel)]);
    pvec.push(func_arg(&sel));
    pvec.push(func_arg(&val));
    pvec.push(func_var(&var, round_width(width.clone())));
    bvec.push(func_assert_lt(&sel, max_sel(width.clone())));
    bvec.push(func_assign(a, RcDoc::as_string(0)));
    bvec.push(func_assign(b.clone(), RcDoc::as_string(path)));
    bvec.push(func_assign(c, RcDoc::as_string(&val)));
    bvec.push(func_assign(RcDoc::as_string(path), b.clone()));
    let prologue = RcDoc::intersperse(pvec.into_iter(), RcDoc::hardline());
    let body = RcDoc::intersperse(bvec.into_iter(), RcDoc::hardline());
    func_write(path, prologue, body)
}

fn func_read_memory<'a>(path: &str, width: u32) -> RcDoc<'a> {
    let var = "data";
    let sel = "sel";
    let addr = "addr";
    let mut pvec = Vec::new();
    let mut bvec = Vec::new();
    let round_index = func_index(
        RcDoc::as_string(0),
        RcDoc::as_string(round_width(width.clone())),
    );
    let index = func_index(RcDoc::as_string(0), RcDoc::as_string(width.clone()));
    let mem_addr = format!("{}[{}]", path, &addr);
    let rval = RcDoc::concat(vec![RcDoc::as_string(&var), func_index_var(&sel)]);
    let a = RcDoc::concat(vec![RcDoc::as_string(&var), round_index]);
    let b = RcDoc::concat(vec![RcDoc::as_string(&var), index]);
    pvec.push(func_arg(&addr));
    pvec.push(func_arg(&sel));
    pvec.push(func_var(&var, round_width(width.clone())));
    bvec.push(func_assert_lt(&sel, max_sel(width.clone())));
    bvec.push(func_assign(a, RcDoc::as_string(0)));
    bvec.push(func_assign(b, RcDoc::as_string(mem_addr)));
    bvec.push(func_return(rval));
    let prologue = RcDoc::intersperse(pvec.into_iter(), RcDoc::hardline());
    let body = RcDoc::intersperse(bvec.into_iter(), RcDoc::hardline());
    func_read(path, prologue, body)
}

fn func_write_memory<'a>(path: &str, width: u32) -> RcDoc<'a> {
    let var = "data";
    let sel = "sel";
    let addr = "addr";
    let val = "value";
    let mut pvec = Vec::new();
    let mut bvec = Vec::new();
    let round_index = func_index(
        RcDoc::as_string(0),
        RcDoc::as_string(round_width(width.clone())),
    );
    let index = func_index(RcDoc::as_string(0), RcDoc::as_string(width.clone()));
    let mem_addr = format!("{}[{}]", path, &addr);
    let a = RcDoc::concat(vec![RcDoc::as_string(&var), round_index]);
    let b = RcDoc::concat(vec![RcDoc::as_string(&var), index]);
    let c = RcDoc::concat(vec![RcDoc::as_string(&var), func_index_var(&sel)]);
    pvec.push(func_arg(&addr));
    pvec.push(func_arg(&sel));
    pvec.push(func_arg(&val));
    pvec.push(func_var(&var, round_width(width.clone())));
    bvec.push(func_assert_lt(&sel, max_sel(width.clone())));
    bvec.push(func_assign(a, RcDoc::as_string(0)));
    bvec.push(func_assign(b.clone(), RcDoc::as_string(&mem_addr)));
    bvec.push(func_assign(c, RcDoc::as_string(&val)));
    bvec.push(func_assign(RcDoc::as_string(&mem_addr), b.clone()));
    let prologue = RcDoc::intersperse(pvec.into_iter(), RcDoc::hardline());
    let body = RcDoc::intersperse(bvec.into_iter(), RcDoc::hardline());
    func_write(path, prologue, body)
}

fn func_read_signature<'a>(prefix: &'a str, args: &Vec<&str>) -> RcDoc<'a> {
    let d = RcDoc::intersperse(args.iter().map(|i| RcDoc::as_string(i)), RcDoc::text(", "));
    let func_name = func_read_name(prefix);
    RcDoc::as_string(func_name).append(func_paren(d))
}

fn func_write_signature<'a>(prefix: &'a str, args: &Vec<&str>) -> RcDoc<'a> {
    let d = RcDoc::intersperse(args.iter().map(|i| RcDoc::as_string(i)), RcDoc::text(", "));
    let func_name = func_write_name(prefix);
    RcDoc::as_string(func_name).append(func_paren(d))
}

fn func_switch_read_register<'a>(prefix: &str, reg: &'a Vec<Register>) -> RcDoc<'a> {
    let hid = "hid";
    let vargs = vec!["sel"];
    let mut pvec = Vec::new();
    let mut bvec = Vec::new();
    pvec.push(func_arg(&hid));
    for v in vargs.iter() {
        pvec.push(func_arg(v));
    }
    for (i, r) in reg.iter().enumerate() {
        if i == 0 {
            bvec.push(func_ifeq(
                &hid,
                r.hid,
                func_return(func_read_signature(&r.path, &vargs)),
            ));
        } else {
            bvec.push(func_else(func_ifeq(
                &hid,
                r.hid,
                func_return(func_read_signature(&r.path, &vargs)),
            )));
        }
    }
    if reg.is_empty() {
        bvec.push(func_error("there is no register declared"));
    } else {
        bvec.push(func_else(func_begin_end(func_error(
            "wrong hid for reading register",
        ))));
    }
    let prologue = RcDoc::intersperse(pvec.into_iter(), RcDoc::hardline());
    let body = RcDoc::concat(bvec);
    func_read(prefix, prologue, body)
}

fn func_switch_write_register<'a>(prefix: &str, reg: &'a Vec<Register>) -> RcDoc<'a> {
    let hid = "hid";
    let vargs = vec!["sel", "value"];
    let mut pvec = Vec::new();
    let mut bvec = Vec::new();
    pvec.push(func_arg(&hid));
    for v in vargs.iter() {
        pvec.push(func_arg(v));
    }
    for (i, r) in reg.iter().enumerate() {
        if i == 0 {
            bvec.push(func_ifeq(
                &hid,
                r.hid,
                func_eval(func_write_signature(&r.path, &vargs)),
            ));
        } else {
            bvec.push(func_else(func_ifeq(
                &hid,
                r.hid,
                func_eval(func_write_signature(&r.path, &vargs)),
            )));
        }
    }
    if reg.is_empty() {
        bvec.push(func_error("there is no register declared"));
    } else {
        bvec.push(func_else(func_begin_end(func_error(
            "wrong hid for writing register",
        ))));
    }
    let prologue = RcDoc::intersperse(pvec.into_iter(), RcDoc::hardline());
    let body = RcDoc::concat(bvec);
    func_write(prefix, prologue, body)
}

fn func_switch_read_memory<'a>(prefix: &str, mem: &'a Vec<Memory>) -> RcDoc<'a> {
    let hid = "hid";
    let vargs = vec!["addr", "sel"];
    let mut pvec = Vec::new();
    let mut bvec = Vec::new();
    pvec.push(func_arg(&hid));
    for v in vargs.iter() {
        pvec.push(func_arg(v));
    }
    for (i, m) in mem.iter().enumerate() {
        if i == 0 {
            bvec.push(func_ifeq(
                &hid,
                m.hid,
                func_return(func_read_signature(&m.path, &vargs)),
            ));
        } else {
            bvec.push(func_else(func_ifeq(
                &hid,
                m.hid,
                func_return(func_read_signature(&m.path, &vargs)),
            )));
        }
    }
    if mem.is_empty() {
        bvec.push(func_error("there is no memory declared"));
    } else {
        bvec.push(func_else(func_begin_end(func_error(
            "wrong hid for reading memory",
        ))));
    }
    let prologue = RcDoc::intersperse(pvec.into_iter(), RcDoc::hardline());
    let body = RcDoc::concat(bvec);
    func_read(prefix, prologue, body)
}

fn func_switch_write_memory<'a>(prefix: &str, mem: &'a Vec<Memory>) -> RcDoc<'a> {
    let hid = "hid";
    let vargs = vec!["addr", "sel", "value"];
    let mut pvec = Vec::new();
    let mut bvec = Vec::new();
    pvec.push(func_arg(&hid));
    for v in vargs.iter() {
        pvec.push(func_arg(v));
    }
    for (i, m) in mem.iter().enumerate() {
        if i == 0 {
            bvec.push(func_ifeq(
                &hid,
                m.hid,
                func_eval(func_write_signature(&m.path, &vargs)),
            ));
        } else {
            bvec.push(func_else(func_ifeq(
                &hid,
                m.hid,
                func_eval(func_write_signature(&m.path, &vargs)),
            )));
        }
    }
    if mem.is_empty() {
        bvec.push(func_error("there is no memory declared"));
    } else {
        bvec.push(func_else(func_begin_end(func_error(
            "wrong hid for writing memory",
        ))));
    }
    let prologue = RcDoc::intersperse(pvec.into_iter(), RcDoc::hardline());
    let body = RcDoc::concat(bvec);
    func_write(prefix, prologue, body)
}

fn func_export<'a>(name: &str) -> RcDoc<'a> {
    RcDoc::text("export")
        .append(RcDoc::space())
        .append(func_str("DPI-C"))
        .append(RcDoc::space())
        .append(RcDoc::text("function"))
        .append(RcDoc::space())
        .append(RcDoc::as_string(name))
        .append(RcDoc::text(";"))
}

impl ToDoc for Register {
    fn to_doc(&self) -> RcDoc<()> {
        RcDoc::concat(vec![
            func_read_register(&self.path, self.width.clone()),
            RcDoc::hardline(),
            func_write_register(&self.path, self.width.clone()),
        ])
    }
}

impl ToDoc for Memory {
    fn to_doc(&self) -> RcDoc<()> {
        RcDoc::concat(vec![
            func_read_memory(&self.path, self.width.clone()),
            RcDoc::hardline(),
            func_write_memory(&self.path, self.width.clone()),
        ])
    }
}

impl ToDoc for LastLayer {
    fn to_doc(&self) -> RcDoc<()> {
        match self {
            LastLayer::AWIG(module_name, reg_name, mem_name, reg, mem) => {
                let mut doc = RcDoc::nil();
                for r in reg.iter() {
                    doc = doc.append(r.to_doc()).append(RcDoc::hardline());
                }
                for m in mem.iter() {
                    doc = doc.append(m.to_doc()).append(RcDoc::hardline());
                }
                doc = doc.append(RcDoc::concat(vec![
                    func_switch_read_register(reg_name, reg),
                    RcDoc::hardline(),
                    func_switch_write_register(reg_name, reg),
                    RcDoc::hardline(),
                    func_switch_read_memory(mem_name, mem),
                    RcDoc::hardline(),
                    func_switch_write_memory(mem_name, mem),
                ]));
                doc = doc.append(RcDoc::hardline());
                doc = doc.append(func_export(&func_read_name(reg_name)));
                doc = doc.append(RcDoc::hardline());
                doc = doc.append(func_export(&func_write_name(reg_name)));
                doc = doc.append(RcDoc::hardline());
                doc = doc.append(func_export(&func_read_name(mem_name)));
                doc = doc.append(RcDoc::hardline());
                doc = doc.append(func_export(&func_write_name(mem_name)));
                func_module(module_name, doc)
            }
        }
    }
}

fn check_register_hid(reg: &Vec<Register>) -> bool {
    let mut map: HashSet<u32> = HashSet::new();
    for r in reg.iter() {
        if !map.contains(&r.hid) {
            map.insert(r.hid);
        } else {
            panic!("register hid already exists");
        }
    }
    true
}

fn check_memory_hid(mem: &Vec<Memory>) -> bool {
    let mut map: HashSet<u32> = HashSet::new();
    for m in mem.iter() {
        if !map.contains(&m.hid) {
            map.insert(m.hid);
        } else {
            panic!("memory hid already exists");
        }
    }
    true
}

pub fn compile(
    path: &Path,
    top_name: &str,
    module_name: &str,
    reg_func_prefix: &str,
    mem_func_prefix: &str,
    reg: &Vec<Register>,
    mem: &Vec<Memory>,
) -> std::io::Result<()> {
    check_register_hid(reg);
    check_memory_hid(mem);
    let mut llreg = reg.clone();
    for (l, r) in llreg.iter_mut().zip(reg.iter()) {
        l.path = format!("{}.{}", top_name, r.path);
    }
    let mut llmem = mem.clone();
    for (l, m) in llmem.iter_mut().zip(mem.iter()) {
        l.path = format!("{}.{}", top_name, m.path);
    }
    let awig = LastLayer::AWIG(
        module_name.to_string(),
        reg_func_prefix.to_string(),
        mem_func_prefix.to_string(),
        llreg,
        llmem,
    );
    let mut file = File::create(path)?;
    file.write_all(awig.to_pretty().as_bytes())?;
    Ok(())
}
