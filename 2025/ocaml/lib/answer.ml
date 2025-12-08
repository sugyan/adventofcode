type answer = IntAnswer of int | StringAnswer of string

let of_int n = IntAnswer n
let of_string s = StringAnswer s
let to_string = function IntAnswer n -> string_of_int n | StringAnswer s -> s
