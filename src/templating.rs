use crate::{filters, functions};

pub fn make_engine() -> tera::Tera {
    let mut tera = tera::Tera::default();
    tera.register_filter("add", filters::add);
    tera.register_filter("sub", filters::sub);
    tera.register_filter("mod", filters::modify);
    tera.register_filter("urlencode_lzma", filters::urlencode_lzma);
    tera.register_function("mix", functions::mix);
    tera.register_function("if", functions::if_fn);
    tera.register_function("object", functions::object);
    tera
}
