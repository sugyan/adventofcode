module type Day = sig
  type input

  val parse : In_channel.t -> input
  val part1 : input -> Answer.answer
  val part2 : input -> Answer.answer
end

type part = Part1 | Part2 | Both

let run ?(part : part = Both) (module D : Day) (channel : In_channel.t) =
  let input = D.parse channel in
  if part = Part1 || part = Both then
    D.part1 input |> Answer.to_string
    |> Printf.sprintf "Part 1: %s"
    |> print_endline;
  if part = Part2 || part = Both then
    D.part2 input |> Answer.to_string
    |> Printf.sprintf "Part 2: %s"
    |> print_endline
