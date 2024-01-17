module Answer = struct
  type t = Integer of int | String of string

  let to_string = function
    | Integer i -> string_of_int i
    | String s -> s

  let of_int i = Integer i
  let of_string s = String s
end

(* type answer = Integer of int | String of string

let answer_of_int i = Integer i
let answer_of_string s = String s *)

module type Solve = sig
  type t

  val parse : in_channel -> t
  val part1 : t -> Answer.t
  val part2 : t -> Answer.t
end
