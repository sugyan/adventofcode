open Base

module Solution : Solution.Solve = struct
  type abc = A | B | C
  type xyz = X | Y | Z
  type t = (abc * xyz) list

  let total_score l score =
    List.map l ~f:score |> List.sum (module Int) ~f:Fn.id

  let parse input =
    let parse_abc = function
      | 'A' -> A
      | 'B' -> B
      | 'C' -> C
      | _ -> failwith "Invalid input"
    in
    let parse_xyz = function
      | 'X' -> X
      | 'Y' -> Y
      | 'Z' -> Z
      | _ -> failwith "Invalid input"
    in
    Stdio.In_channel.input_lines input
    |> List.map ~f:(fun line ->
           (String.get line 0 |> parse_abc, String.get line 2 |> parse_xyz))

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

  let part1 t = Solution.Integer (total_score t score_part1)
  let part2 t = Solution.Integer (total_score t score_part2)
end
