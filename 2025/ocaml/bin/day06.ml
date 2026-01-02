open Aoc2025
open Base

module Solution : Aoc.Day = struct
  type op = Add | Mul
  type input = char list list * op list

  let parse channel =
    let lines = In_channel.input_lines channel in
    ( List.drop_last_exn lines |> List.map ~f:String.to_list,
      List.last_exn lines |> String.split ~on:' '
      |> List.filter ~f:(String.is_empty |> Fn.non)
      |> List.map ~f:(function
        | "+" -> Add
        | "*" -> Mul
        | _ -> failwith "invalid op") )

  let parse_numbers lst =
    String.of_list lst |> String.split ~on:' '
    |> List.filter ~f:(String.is_empty |> Fn.non)
    |> List.map ~f:Int.of_string

  let perform (op, nums) =
    List.reduce_exn nums ~f:(match op with Add -> ( + ) | Mul -> ( * ))

  let part1 input =
    fst input |> List.map ~f:parse_numbers |> List.transpose_exn
    |> List.zip_exn (snd input)
    |> List.sum (module Int) ~f:perform
    |> Answer.of_int

  let part2 input =
    fst input |> List.transpose_exn |> List.map ~f:parse_numbers
    |> List.group ~break:(fun lst _ -> List.is_empty lst)
    |> List.map ~f:List.concat
    |> List.zip_exn (snd input)
    |> List.sum (module Int) ~f:perform
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
