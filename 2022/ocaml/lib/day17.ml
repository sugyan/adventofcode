open Base
open Solution

module Solution : Solve = struct
  type t = int -> int
  type jet = L | R

  let rocks =
    [
      ([ (0, 0); (1, 0); (2, 0); (3, 0) ], 0);
      ([ (1, 0); (0, 1); (1, 1); (2, 1); (1, 2) ], 2);
      ([ (0, 0); (1, 0); (2, 0); (2, 1); (2, 2) ], 2);
      ([ (0, 0); (0, 1); (0, 2); (0, 3) ], 3);
      ([ (0, 0); (1, 0); (0, 1); (1, 1) ], 1);
    ]

  let parse input =
    let jets =
      Stdio.In_channel.input_line_exn input
      |> String.to_list
      |> List.map ~f:(function
           | '<' -> L
           | '>' -> R
           | _ -> failwith "invalid input")
    in
    fun num_rocks ->
      let to_nth lst =
        let a, l = List.(to_array lst, length lst) in
        fun i -> a.(i % l)
      in
      let rock_of = to_nth rocks in
      let jet_of = to_nth jets in
      let tower = Hash_set.create (module Utils.XY) in
      let can_place rock (dx, dy) =
        List.map rock ~f:(fun (x, y) -> (x + dx, y + dy))
        |> List.for_all ~f:(fun (x, y) ->
               x >= 0 && x < 7 && y > 0 && not (Hash_set.mem tower (x, y)))
      in
      let rec loop n j top =
        if n = num_rocks then top
        else
          let r, h = rock_of n in
          let c = can_place r in
          let rec fall (x, y) i =
            ( (match jet_of (j + i) with L -> (x - 1, y) | R -> (x + 1, y))
            |> fun d -> if c d then d else (x, y) )
            |> fun (x, y) ->
            if c (x, y - 1) then fall (x, y - 1) (i + 1) else ((x, y), i + 1)
          in
          let (px, py), dj = fall (2, top + 4) 0 in
          List.map r ~f:(fun (x, y) -> (x + px, y + py))
          |> List.iter ~f:(Hash_set.add tower);
          loop (n + 1) (j + dj) (max top (py + h))
      in
      loop 0 0 0

  let part1 tower_height = tower_height 2022 |> answer_of_int
  let part2 _ = failwith "not implemented"
end
