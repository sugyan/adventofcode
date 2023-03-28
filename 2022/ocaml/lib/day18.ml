open Base
open Solution

module Solution : Solve = struct
  type t = (int * int * int) Hash_set.t

  let adjacents (x, y, z) =
    [ (-1, 0, 0); (1, 0, 0); (0, -1, 0); (0, 1, 0); (0, 0, -1); (0, 0, 1) ]
    |> List.map ~f:(fun (dx, dy, dz) -> (x + dx, y + dy, z + dz))
    |> Hash_set.of_list (module Utils.XYZ)

  let count ~f =
    Hash_set.sum (module Int) ~f:(fun c -> adjacents c |> Hash_set.count ~f)

  let parse input =
    let parse_line line =
      String.split line ~on:',' |> List.map ~f:Int.of_string |> function
      | [ x; y; z ] -> (x, y, z)
      | _ -> failwith "invalid input"
    in
    Stdio.In_channel.input_lines input
    |> List.map ~f:parse_line
    |> Hash_set.of_list (module Utils.XYZ)

  let part1 cubes =
    count cubes ~f:(Hash_set.mem cubes |> Fn.non) |> answer_of_int

  let part2 cubes =
    let in_range =
      let max_x, max_y, max_z =
        Hash_set.fold cubes ~init:(0, 0, 0) ~f:(fun (x, y, z) (x', y', z') ->
            (max x x', max y y', max z z'))
      in
      let ok v max = -1 <= v && v <= max + 1 in
      fun (x, y, z) -> ok x max_x && ok y max_y && ok z max_z
    in
    let seen = Hash_set.create (module Utils.XYZ) in
    let q = Queue.create () in
    let rec bfs = function
      | Some p ->
          adjacents p
          |> Hash_set.filter ~f:in_range
          |> Fn.flip Hash_set.diff seen
          |> Fn.flip Hash_set.diff cubes
          |> Hash_set.iter ~f:(fun c ->
                 Hash_set.add seen c;
                 Queue.enqueue q c);
          bfs (Queue.dequeue q)
      | None -> count cubes ~f:(Hash_set.mem seen)
    in
    bfs (Some (-1, -1, -1)) |> answer_of_int
end
