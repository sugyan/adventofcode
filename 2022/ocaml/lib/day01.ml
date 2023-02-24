open Base

module Solution : Solution.Solve = struct
  type t = int -> int

  let parse input =
    let rec split_by_blank_line xs =
      List.split_while xs ~f:(Fn.non String.is_empty) |> function
      | hd, [] -> [ hd ]
      | hd, _ :: tl -> hd :: split_by_blank_line tl
    in
    let total_calories =
      Stdio.In_channel.input_lines input
      |> split_by_blank_line
      |> List.map ~f:(List.sum (module Int) ~f:Int.of_string)
      |> List.sort ~compare:Int.descending
    in
    fun n -> List.take total_calories n |> List.sum (module Int) ~f:Fn.id

  let part1 top_n_sum = top_n_sum 1 |> Solution.answer_of_int
  let part2 top_n_sum = top_n_sum 3 |> Solution.answer_of_int
end
