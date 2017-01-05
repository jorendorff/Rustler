use rustler::{NifEnv, NifTerm};

mod test_primitives;
use test_primitives::{add_u32, add_i32, tuple_add, echo_u8};

mod test_list;
use test_list::{sum_list, make_list};

mod test_map;
use test_map::{sum_map_values, map_entries_sorted};

mod test_resource;
use test_resource::{resource_make, resource_set_integer_field, resource_get_integer_field};

mod test_binary;
use test_binary::make_shorter_subbinary;

mod test_atom;
use test_atom::{atom_to_string};

rustler_export_nifs!(
    "Elixir.RustlerTest",
    [("add_u32", 2, add_u32),
     ("add_i32", 2, add_i32),
     ("tuple_add", 1, tuple_add),
     ("echo_u8", 1, echo_u8),
     ("sum_list", 1, sum_list),
     ("make_list", 0, make_list),
     ("sum_map_values", 1, sum_map_values),
     ("map_entries_sorted", 1, map_entries_sorted),
     ("resource_make", 0, resource_make),
     ("resource_set_integer_field", 2, resource_set_integer_field),
     ("resource_get_integer_field", 1, resource_get_integer_field),
     ("atom_to_string", 1, atom_to_string),
     ("make_shorter_subbinary", 1, make_shorter_subbinary)],
    Some(on_load)
);

fn on_load(env: NifEnv, _load_info: NifTerm) -> bool {
    test_resource::on_load(env);
    test_atom::on_load(env);
    true
}
