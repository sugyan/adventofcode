open Base
open Solution

module Solution : Solve = struct
  type t = char list list

  let find_digit_with_spelled = function
    | 'o' :: 'n' :: 'e' :: _ -> Some 1
    | 't' :: 'w' :: 'o' :: _ -> Some 2
    | 't' :: 'h' :: 'r' :: 'e' :: 'e' :: _ -> Some 3
    | 'f' :: 'o' :: 'u' :: 'r' :: _ -> Some 4
    | 'f' :: 'i' :: 'v' :: 'e' :: _ -> Some 5
    | 's' :: 'i' :: 'x' :: _ -> Some 6
    | 's' :: 'e' :: 'v' :: 'e' :: 'n' :: _ -> Some 7
    | 'e' :: 'i' :: 'g' :: 'h' :: 't' :: _ -> Some 8
    | 'n' :: 'i' :: 'n' :: 'e' :: _ -> Some 9
    | _ -> None

  let find_digit include_spelled = function
    | ('1' .. '9' as c) :: _ -> Some Char.(to_int c - to_int '0')
    | l -> if include_spelled then find_digit_with_spelled l else None

  let calibration_value include_spelled line =
    let rec f = function
      | [] -> []
      | hd :: tl -> find_digit include_spelled (hd :: tl) :: f tl
    in
    let digits = f line |> List.filter_map ~f:Fn.id in
    List.((10 * hd_exn digits) + last_exn digits)

  let parse input = In_channel.input_lines input |> List.map ~f:String.to_list

  let part1 lines =
    lines |> List.sum (module Int) ~f:(calibration_value false) |> Answer.of_int

  let part2 lines =
    lines |> List.sum (module Int) ~f:(calibration_value true) |> Answer.of_int
end
