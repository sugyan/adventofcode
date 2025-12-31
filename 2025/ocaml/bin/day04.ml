open Aoc2025
open Core

module Solution : Aoc.Day = struct
  type input = (int * int) Hash_set.t

  let dirs =
    [ (-1, -1); (-1, 0); (-1, 1); (0, -1); (0, 1); (1, -1); (1, 0); (1, 1) ]

  let parse channel =
    In_channel.input_lines channel
    |> List.mapi ~f:(fun i line ->
        let f j = function
          | '@' -> Some (Tuple2.create i j)
          | '.' -> None
          | _ -> failwith "invalid input"
        in
        String.to_list line |> List.filter_mapi ~f)
    |> List.concat |> Hash_set.Poly.of_list

  let removable_rolls hs =
    Hash_set.filter hs ~f:(fun (x, y) ->
        List.map dirs ~f:(Tuple2.map2 (x, y) ~f:( + ))
        |> List.count ~f:(Hash_set.mem hs)
        < 4)

  let part1 input = removable_rolls input |> Hash_set.length |> Answer.of_int

  let part2 input =
    Sequence.unfold ~init:input ~f:(fun hs ->
        let removable = removable_rolls hs in
        match Hash_set.length removable with
        | 0 -> None
        | len -> Some (len, Hash_set.diff hs removable))
    |> Sequence.sum (module Int) ~f:Fn.id
    |> Answer.of_int
end

let () =
  let part =
    Sys.get_argv () |> Array.last |> function
    | "part1" -> Aoc.Part1
    | "part2" -> Aoc.Part2
    | _ -> Aoc.Both
  in
  Aoc.run ~part (module Solution) In_channel.stdin
