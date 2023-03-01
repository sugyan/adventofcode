open Base
open Solution

module Solution : Solve = struct
  type state = Down of (int * int) | Rest | Void
  type t = state -> int

  let parse input =
    let cave =
      let parse_line line =
        let parse_xy s =
          let x, y = String.lsplit2_exn s ~on:',' in
          (Int.of_string x, Int.of_string y)
        in
        String.split line ~on:' '
        |> List.filter ~f:(String.( <> ) "->")
        |> List.map ~f:parse_xy |> Utils.pairs
        |> List.map ~f:(fun ((x0, y0), (x1, y1)) ->
               List.cartesian_product
                 (List.range (min x0 x1) (max x0 x1) ~stop:`inclusive)
                 (List.range (min y0 y1) (max y0 y1) ~stop:`inclusive))
        |> List.concat
        |> Hash_set.of_list (module Utils.XY)
      in
      Stdio.In_channel.input_lines input
      |> List.map ~f:parse_line
      |> List.fold ~init:(Hash_set.create (module Utils.XY)) ~f:Hash_set.union
    in
    let ymax =
      Hash_set.to_list cave |> List.map ~f:snd |> List.fold ~init:0 ~f:max
    in
    fun state ->
      let block = Hash_set.copy cave in
      let rec dfs = function
        | [] -> Hash_set.length block - Hash_set.length cave
        | hd :: tl ->
            hd
            |> (fun (x, y) -> [ (x, y + 1); (x - 1, y + 1); (x + 1, y + 1) ])
            |> List.find ~f:(Hash_set.mem block |> Fn.non)
            |> (function
                 | Some (_, y) when y > ymax + 1 -> state
                 | Some p -> Down p
                 | None -> Rest)
            |> (function
                 | Down p -> p :: hd :: tl
                 | Rest ->
                     Hash_set.add block hd;
                     tl
                 | Void -> [])
            |> dfs
      in
      dfs [ (500, 0) ]

  let part1 count_units = count_units Void |> answer_of_int
  let part2 count_units = count_units Rest |> answer_of_int
end
