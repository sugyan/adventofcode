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
      let aor, aoj = (Array.of_list rocks, Array.of_list jets) in
      let len = Array.length aoj in
      let tower = Hash_set.create (module Utils.XY) in
      let cache = Hashtbl.create (module Int) in
      let rec loop n j top =
        let key = (j % len * 5) + (n % 5) in
        Hashtbl.find cache key |> function
        | Some (n', top') when (num_rocks - n) % (n - n') = 0 ->
            top + ((top - top') * ((num_rocks - n) / (n - n')))
        | _ ->
            Hashtbl.set cache ~key ~data:(n, top);
            let r, h = aor.(n % 5) in
            let c (dx, dy) =
              List.map r ~f:(fun (x, y) -> (x + dx, y + dy))
              |> List.for_all ~f:(fun (x, y) ->
                     x >= 0 && x < 7 && y > 0 && not (Hash_set.mem tower (x, y)))
            in
            let rec fall (x, y) i =
              ( (match aoj.((j + i) % len) with
                | L -> (x - 1, y)
                | R -> (x + 1, y))
              |> fun d -> if c d then d else (x, y) )
              |> fun (x, y) ->
              if c (x, y - 1) then fall (x, y - 1) (i + 1) else ((x, y), i + 1)
            in
            let (x', y'), j' = fall (2, top + 4) 0 in
            List.map r ~f:(fun (x, y) -> (x + x', y + y'))
            |> List.iter ~f:(Hash_set.add tower);
            loop (n + 1) (j + j') (max top (y' + h))
      in
      loop 0 0 0

  let part1 tower_height = tower_height 2022 |> answer_of_int
  let part2 tower_height = tower_height 1_000_000_000_000 |> answer_of_int
end
