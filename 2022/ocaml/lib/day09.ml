open Base

module Solution : Solution.Solve = struct
  type t = int -> int

  module XY = struct
    type t = int * int [@@deriving compare, sexp_of, hash]
  end

  let dup x = (x, x)

  let parse input =
    let motions =
      let parse_line s =
        String.split s ~on:' ' |> function
        | [ dir; steps ] ->
            ( (match dir with
              | "U" -> (0, 1)
              | "D" -> (0, -1)
              | "L" -> (-1, 0)
              | "R" -> (1, 0)
              | _ -> failwith "invalid direction"),
              Int.of_string steps )
        | _ -> failwith "invalid line"
      in
      Stdio.In_channel.input_lines input |> List.map ~f:parse_line
    in
    fun n ->
      let do_motions acc (dir, steps) =
        let do_move (knots, (x, y)) (dx, dy) =
          let hd = (x + dx, y + dy) in
          let move_knot acc (x, y) =
            let dx, dy = (fst acc - x, snd acc - y) in
            (if Int.abs dx < 2 && Int.abs dy < 2 then (x, y)
            else (x + Int.compare dx 0, y + Int.compare dy 0))
            |> dup
          in
          let tl, knots = List.fold_map knots ~init:hd ~f:move_knot in
          ((knots, hd), tl)
        in
        List.init steps ~f:(Fn.const dir) |> List.fold_map ~init:acc ~f:do_move
      in
      let knots = List.init (n - 1) ~f:(Fn.const (0, 0)) in
      motions
      |> List.folding_map ~init:(knots, (0, 0)) ~f:do_motions
      |> List.concat
      |> Hash_set.of_list (module XY)
      |> Hash_set.length

  let part1 tail_visited = tail_visited 2 |> Solution.answer_of_int
  let part2 tail_visited = tail_visited 10 |> Solution.answer_of_int
end
