open Base
open Solution

module Solution : Solve = struct
  type abc = A | B | C
  type xyz = X | Y | Z
  type t = (abc * xyz -> int) -> int

  let parse input =
    let to_abc = function
      | 'A' -> A
      | 'B' -> B
      | 'C' -> C
      | _ -> failwith "invalid input"
    in
    let to_xyz = function
      | 'X' -> X
      | 'Y' -> Y
      | 'Z' -> Z
      | _ -> failwith "invalid input"
    in
    let parse_line s =
      String.to_list s |> function
      | [ c0; ' '; c2 ] -> (to_abc c0, to_xyz c2)
      | _ -> failwith "invalid input"
    in
    let guide = Stdio.In_channel.input_lines input |> List.map ~f:parse_line in
    fun f -> guide |> List.sum (module Int) ~f

  let part1 total =
    total (function
      | A, X -> 4 (* 1 + 3 *)
      | A, Y -> 8 (* 2 + 6 *)
      | A, Z -> 3 (* 3 + 0 *)
      | B, X -> 1 (* 1 + 0 *)
      | B, Y -> 5 (* 2 + 3 *)
      | B, Z -> 9 (* 3 + 6 *)
      | C, X -> 7 (* 1 + 6 *)
      | C, Y -> 2 (* 2 + 0 *)
      | C, Z -> 6 (* 3 + 3 *))
    |> answer_of_int

  let part2 total =
    total (function
      | A, X -> 3 (* 3 + 0 *)
      | A, Y -> 4 (* 1 + 3 *)
      | A, Z -> 8 (* 2 + 6 *)
      | B, X -> 1 (* 1 + 0 *)
      | B, Y -> 5 (* 2 + 3 *)
      | B, Z -> 9 (* 3 + 6 *)
      | C, X -> 2 (* 2 + 0 *)
      | C, Y -> 6 (* 3 + 3 *)
      | C, Z -> 7 (* 1 + 6 *))
    |> answer_of_int
end
