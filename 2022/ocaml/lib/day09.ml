open Base

module Solution : Solution.Solve = struct
  type t = int -> int

  module XY = struct
    type t = int * int [@@deriving compare, sexp_of, hash]
  end

  let dup x = (x, x)

  let parse input =
    let heads =
      let parse_line s =
        String.split s ~on:' ' |> function
        | [ dir; steps ] ->
            ( (match dir with
              | "U" -> (0, 1)
              | "D" -> (0, -1)
              | "L" -> (-1, 0)
              | "R" -> (1, 0)
              | _ -> failwith "Invalid direction"),
              Int.of_string steps )
        | _ -> failwith "Invalid line"
      in
      let make_list (d, n) = List.init n ~f:(fun _ -> d) in
      let move hd x = (fst hd + fst x, snd hd + snd x) |> dup in
      Stdio.In_channel.input_lines input
      |> List.map ~f:parse_line |> List.map ~f:make_list |> List.concat
      |> List.folding_map ~init:(0, 0) ~f:move
    in
    fun n ->
      let move knots hd =
        let move_knot acc (x, y) =
          let dx, dy = (fst acc - x, snd acc - y) in
          (if Int.abs dx < 2 && Int.abs dy < 2 then (x, y)
          else (x + Int.compare dx 0, y + Int.compare dy 0))
          |> dup
        in
        let tl, knots = List.fold_map knots ~init:hd ~f:move_knot in
        (knots, tl)
      in
      heads
      |> List.folding_map ~init:(List.init (n - 1) ~f:(fun _ -> (0, 0))) ~f:move
      |> Hash_set.of_list (module XY)
      |> Hash_set.length

  let part1 tail_visited = tail_visited 2 |> Solution.answer_of_integer
  let part2 tail_visited = tail_visited 10 |> Solution.answer_of_integer
end
