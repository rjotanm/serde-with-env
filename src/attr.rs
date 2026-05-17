use syn::{Attribute, ExprLit, LitStr};

pub enum EnvAttrOp {
    Or,
    Only,
    Over,
}
pub struct EnvAttr {
    pub name: LitStr,
    pub default: Option<ExprLit>,
    pub convert: Option<LitStr>,
    pub op: EnvAttrOp,
}

pub fn get_env_attr(attrs: &Vec<Attribute>) -> Option<Result<EnvAttr, syn::Error>> {
    let mut maybe_attr: Option<Result<EnvAttr, syn::Error>> = None;
    let mut maybe_default: Option<ExprLit> = None;
    let mut maybe_convert: Option<LitStr> = None;

    for attr in attrs {
        if attr.path().is_ident("with_env") {
            let result = attr.parse_nested_meta(|meta| {
                if meta.path.is_ident("default") {
                    _ = meta
                        .value()
                        .and_then(|value| value.parse::<ExprLit>())
                        .map(|value| maybe_default = Some(value));
                }

                if meta.path.is_ident("convert") {
                    _ = meta
                        .value()
                        .and_then(|value| value.parse::<LitStr>())
                        .map(|value| maybe_convert = Some(value));
                }


                let mut op = None;

                if meta.path.is_ident("or") {
                    op = Some(EnvAttrOp::Or);
                }
                if meta.path.is_ident("only") {
                    op = Some(EnvAttrOp::Only);
                }
                if meta.path.is_ident("over") {
                    op = Some(EnvAttrOp::Over);
                }

                if let Some(op) = op {
                    meta.value()
                        .and_then(|value| value.parse::<LitStr>())
                        .map(|value| {
                            maybe_attr = Some(Ok(EnvAttr {
                                name: value,
                                default: None,
                                convert: None,
                                op,
                            }));

                            ()
                        })
                } else {
                    Ok(())
                }
            });

            if let Err(err) = result {
                maybe_attr = Some(Err(err))
            }
        }
    }
    if let Some(maybe_default) = maybe_default
        && let Some(Ok(attr)) = &mut maybe_attr
    {
        attr.default = Some(maybe_default);
    }

    if let Some(maybe_convert) = maybe_convert
        && let Some(Ok(attr)) = &mut maybe_attr
    {
        attr.convert = Some(maybe_convert);
    }

    maybe_attr
}
