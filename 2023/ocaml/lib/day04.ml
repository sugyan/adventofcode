open Base
open Solution

module Solution : Solve = struct
  type t = int list

  let parse input =
    let match_count line =
      let to_int_list s =
        String.split s ~on:' '
        |> List.filter ~f:(Fn.non String.is_empty)
        |> List.map ~f:Int.of_string
      in
      String.lsplit2_exn line ~on:':' |> snd |> String.lsplit2_exn ~on:'|'
      |> fun (winning, have) ->
      let w = winning |> to_int_list in
      have |> to_int_list |> List.count ~f:(List.mem w ~equal)
    in
    In_channel.input_lines input |> List.map ~f:match_count

  let part1 matches =
    List.sum (module Int) matches ~f:(fun x -> (1 lsl x) lsr 1) |> Answer.of_int

  let part2 matches =
    let a = Array.create ~len:(List.length matches) 1 in
    List.iteri matches ~f:(fun i x ->
        List.range (i + 1) (i + 1 + x)
        |> List.iter ~f:(fun j -> a.(j) <- a.(j) + a.(i)));
    Array.fold a ~init:0 ~f:( + ) |> Answer.of_int
end
