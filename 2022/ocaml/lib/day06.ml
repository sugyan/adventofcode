open Base

module Solution : Solution.Solve = struct
  type t = int -> int

  let parse input =
    let buffer = Stdio.In_channel.input_line_exn input |> String.to_list in
    fun n ->
      let rec loop l i =
        if List.take l n |> Set.of_list (module Char) |> Set.length = n then i
        else loop (List.tl_exn l) i + 1
      in
      loop buffer n

  let part1 processed_count = Solution.Integer (processed_count 4)
  let part2 processed_count = Solution.Integer (processed_count 14)
end
