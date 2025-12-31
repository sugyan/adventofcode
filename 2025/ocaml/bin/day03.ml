open Aoc2025
open Base

module Solution : Aoc.Day = struct
  type input = int array list

  let parse channel =
    let parse_line s =
      String.to_array s
      |> Array.map ~f:(fun c -> Char.to_string c |> Int.of_string)
    in
    In_channel.input_lines channel |> List.map ~f:parse_line

  let max_n_digit_number batteries digits =
    let rec aux n lst =
      if n = batteries then List.last_exn lst
      else
        let f i acc x =
          (x * 10) + digits.(n + i) |> max acc |> fun m -> (m, m)
        in
        List.drop_last_exn lst |> List.folding_mapi ~init:0 ~f |> aux (n + 1)
    in
    List.init (Array.length digits + 1) ~f:(Fn.const 0) |> aux 0

  let part1 input =
    List.sum (module Int) input ~f:(max_n_digit_number 2) |> Answer.of_int

  let part2 input =
    List.sum (module Int) input ~f:(max_n_digit_number 12) |> Answer.of_int
end

let () =
  let part =
    Sys.get_argv () |> Array.last |> function
    | "part1" -> Aoc.Part1
    | "part2" -> Aoc.Part2
    | _ -> Aoc.Both
  in
  Aoc.run ~part (module Solution) In_channel.stdin
