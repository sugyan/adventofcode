type answer = Integer of int | String of string

let answer_of_int i = Integer i
let answer_of_string i = String i

module type Solve = sig
  type t

  val parse : in_channel -> t
  val part1 : t -> answer
  val part2 : t -> answer
end
