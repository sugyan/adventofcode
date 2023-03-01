open Base
open Solution

module Solution : Solve = struct
  type t = int -> int

  let parse input =
    let motions =
      let parse_line s =
        String.split s ~on:' ' |> function
        | [ dir; steps ] ->
            ( (match dir with
              | "U" -> fun (x, y) -> (x, y + 1)
              | "D" -> fun (x, y) -> (x, y - 1)
              | "L" -> fun (x, y) -> (x - 1, y)
              | "R" -> fun (x, y) -> (x + 1, y)
              | _ -> failwith "invalid direction"),
              Int.of_string steps )
        | _ -> failwith "invalid line"
      in
      Stdio.In_channel.input_lines input |> List.map ~f:parse_line
    in
    fun n ->
      let do_motions acc (dir, steps) =
        let do_move knots () =
          let hd = List.hd_exn knots |> dir in
          let move_knot (prev_x, prev_y) (x, y) =
            let dx, dy = (prev_x - x, prev_y - y) in
            (if abs dx < 2 && abs dy < 2 then (x, y)
            else (x + compare dx 0, y + compare dy 0))
            |> fun p -> (p, p)
          in
          List.tl_exn knots |> List.fold_map ~init:hd ~f:move_knot
          |> fun (last, tl) -> (hd :: tl, last)
        in
        List.init steps ~f:ignore |> List.fold_map ~init:acc ~f:do_move
      in
      let knots = List.init n ~f:(Fn.const (0, 0)) in
      motions
      |> List.folding_map ~init:knots ~f:do_motions
      |> List.concat
      |> Hash_set.of_list (module Utils.XY)
      |> Hash_set.length

  let part1 tail_visited = tail_visited 2 |> answer_of_int
  let part2 tail_visited = tail_visited 10 |> answer_of_int
end
