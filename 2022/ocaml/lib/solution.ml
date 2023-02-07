type answer = Integer of int | String of string

module type Solve = sig
  type t

  val parse : in_channel -> t
  val part1 : t -> answer
  val part2 : t -> answer
end
