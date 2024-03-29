open Base
open Solution

module Solution : Solve = struct
  type value = Integer of int | List of value list
  type t = (value * value) list

  let parse_packet line =
    let c2i c = Char.to_int c - Char.to_int '0' in
    let rec loop acc = function
      | '1' :: '0' :: tl -> loop (Integer 10 :: acc) tl
      | ('0' .. '9' as c) :: tl -> loop (Integer (c2i c) :: acc) tl
      | '[' :: tl ->
          let l, rest = loop [] tl in
          loop (List l :: acc) rest
      | ']' :: tl -> (acc |> List.rev, tl)
      | ',' :: tl -> loop acc tl
      | [] -> (acc, [])
      | _ -> failwith "invalid input"
    in
    loop [] (String.to_list line) |> fst |> List.hd_exn

  let ( < ) v0 v1 =
    let rec cmp = function
      | Integer i0, Integer i1 -> compare i0 i1
      | List l0, List l1 -> cmp_lst (l0, l1)
      | i, List l -> cmp_lst ([ i ], l)
      | List l, i -> cmp_lst (l, [ i ])
    and cmp_lst = function
      | [], [] -> 0
      | [], _ -> -1
      | _, [] -> 1
      | hd0 :: tl0, hd1 :: tl1 -> (
          cmp (hd0, hd1) |> function 0 -> cmp_lst (tl0, tl1) | o -> o)
    in
    cmp (v0, v1) < 0

  let parse input =
    let parse_chunks = function
      | [ left; right ] -> (parse_packet left, parse_packet right)
      | _ -> failwith "invalid input"
    in
    Stdio.In_channel.input_lines input
    |> List.filter ~f:(Fn.non String.is_empty)
    |> List.chunks_of ~length:2 |> List.map ~f:parse_chunks

  let part1 pairs =
    pairs
    |> List.filter_mapi ~f:(fun i (l, r) ->
           if l < r then Some (i + 1) else None)
    |> List.sum (module Int) ~f:Fn.id
    |> answer_of_int

  let part2 pairs =
    let packets =
      pairs |> List.map ~f:(fun pair -> [ fst pair; snd pair ]) |> List.concat
    in
    [ "[[2]]"; "[[6]]" ] |> List.map ~f:parse_packet
    |> List.mapi ~f:(fun i p -> i + 1 + List.count packets ~f:(Fn.flip ( < ) p))
    |> List.fold ~init:1 ~f:( * ) |> answer_of_int
end
