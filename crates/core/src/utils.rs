use ckb_gen_types::packed::Script;

pub fn is_script_code_equal(a: &Script, b: &Script) -> bool {
    a.code_hash() == b.code_hash() && a.hash_type() == b.hash_type()
}
