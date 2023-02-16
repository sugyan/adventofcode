open Base

module Solution : Solution.Solve = struct
  type abc = A | B | C
  type xyz = X | Y | Z
  type t = (abc * xyz -> int) -> int

  let parse input =
    let parse_abc = function
      | 'A' -> A
      | 'B' -> B
      | 'C' -> C
      | _ -> failwith "invalid input"
    in
    let parse_xyz = function
      | 'X' -> X
      | 'Y' -> Y
      | 'Z' -> Z
      | _ -> failwith "invalid input"
    in
    let parse_line s = (s.[0] |> parse_abc, s.[2] |> parse_xyz) in
    let guide = Stdio.In_channel.input_lines input |> List.map ~f:parse_line in
    fun f -> guide |> List.sum (module Int) ~f

  let score_part1 = function
    | A, X -> 4 (* 1 + 3 *)
    | A, Y -> 8 (* 2 + 6 *)
    | A, Z -> 3 (* 3 + 0 *)
    | B, X -> 1 (* 1 + 0 *)
    | B, Y -> 5 (* 2 + 3 *)
    | B, Z -> 9 (* 3 + 6 *)
    | C, X -> 7 (* 1 + 6 *)
    | C, Y -> 2 (* 2 + 0 *)
    | C, Z -> 6 (* 3 + 3 *)

  let score_part2 = function
    | A, X -> 3 (* 3 + 0 *)
    | A, Y -> 4 (* 1 + 3 *)
    | A, Z -> 8 (* 2 + 6 *)
    | B, X -> 1 (* 1 + 0 *)
    | B, Y -> 5 (* 2 + 3 *)
    | B, Z -> 9 (* 3 + 6 *)
    | C, X -> 2 (* 2 + 0 *)
    | C, Y -> 6 (* 3 + 3 *)
    | C, Z -> 7 (* 1 + 6 *)

  let part1 total = total score_part1 |> Solution.answer_of_int
  let part2 total = total score_part2 |> Solution.answer_of_int
end
