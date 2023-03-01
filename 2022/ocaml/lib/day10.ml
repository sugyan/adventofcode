open Base
open Solution

module Solution : Solve = struct
  type t = int list

  let parse input =
    let tick acc x = (acc + x, acc) in
    Stdio.In_channel.input_lines input
    |> List.map ~f:(String.split ~on:' ')
    |> List.map ~f:(function
         | [ "noop" ] -> [ 0 ]
         | [ "addx"; x ] -> [ 0; Int.of_string x ]
         | _ -> failwith "invalid input")
    |> List.concat
    |> List.folding_map ~init:1 ~f:tick

  let part1 values =
    let strength i = i * List.nth_exn values (i - 1) in
    [ 20; 60; 100; 140; 180; 220 ]
    |> List.sum (module Int) ~f:strength
    |> answer_of_int

  let part2 values =
    let row chunk =
      let pixel i x = if i - x |> abs < 2 then '#' else '.' in
      List.mapi chunk ~f:pixel |> String.of_char_list |> ( ^ ) "\n"
    in
    values |> List.chunks_of ~length:40 |> List.map ~f:row |> String.concat
    |> answer_of_string
end
