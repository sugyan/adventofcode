open Aoc2025
open Base

module Solution : Aoc.Day = struct
  type rotation = Left | Right
  type input = (rotation * int) list

  let parse channel =
    let parse_line s =
      let rot =
        match String.get s 0 with
        | 'L' -> Left
        | 'R' -> Right
        | _ -> failwith "invalid input"
      in
      (rot, String.subo s ~pos:1 |> Int.of_string)
    in
    In_channel.input_lines channel |> List.map ~f:parse_line

  let count_zeros lst =
    let f acc (rot, n) =
      let next =
        (match rot with Left -> acc + n | Right -> acc - n)
        |> Fn.flip ( % ) 100
      in
      (next, next)
    in
    List.folding_map lst ~init:50 ~f |> List.count ~f:(( = ) 0)

  let part1 input = count_zeros input |> Answer.of_int

  let part2 input =
    List.map input ~f:(fun (rot, n) -> List.init n ~f:(fun _ -> (rot, 1)))
    |> List.concat |> count_zeros |> Answer.of_int
end

let () =
  let part =
    Sys.get_argv () |> Array.last |> function
    | "part1" -> Aoc.Part1
    | "part2" -> Aoc.Part2
    | _ -> Aoc.Both
  in
  Aoc.run ~part (module Solution) In_channel.stdin
