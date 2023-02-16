open Base

module Solution : Solution.Solve = struct
  type t = int -> int

  let parse input =
    let buffer = Stdio.In_channel.input_line_exn input |> String.to_list in
    fun n ->
      let rec loop l i =
        if List.take l n |> List.contains_dup ~compare:Char.compare then
          loop (List.tl_exn l) i + 1
        else i
      in
      loop buffer n

  let part1 processed_count = processed_count 4 |> Solution.answer_of_int
  let part2 processed_count = processed_count 14 |> Solution.answer_of_int
end
