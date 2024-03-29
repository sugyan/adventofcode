open Base
open Solution

module Solution : Solve = struct
  type t = int list list

  let parse input =
    let priority = function
      | 'a' .. 'z' as c -> Char.to_int c - 96
      | 'A' .. 'Z' as c -> Char.to_int c - 38
      | _ -> failwith "invalid char"
    in
    Stdio.In_channel.input_lines input
    |> List.map ~f:(Fn.compose (List.map ~f:priority) String.to_list)

  let part1 items =
    let f l =
      let half = List.split_n l (List.length l / 2) in
      Set.inter
        (fst half |> Set.of_list (module Int))
        (snd half |> Set.of_list (module Int))
      |> Set.choose_exn
    in
    items |> List.sum (module Int) ~f |> answer_of_int

  let part2 items =
    let f l =
      List.map l ~f:(Set.of_list (module Int))
      |> List.reduce_exn ~f:Set.inter
      |> Set.choose_exn
    in
    items |> List.chunks_of ~length:3
    |> List.sum (module Int) ~f
    |> answer_of_int
end
