open Base

module Solution : Solution.Solve = struct
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
      | Integer i0, Integer i1 -> Int.compare i0 i1
      | Integer i, l -> cmp (List [ Integer i ], l)
      | l, Integer i -> cmp (l, List [ Integer i ])
      | List l0, List l1 ->
          let rec loop = function
            | [], [] -> 0
            | [], _ -> -1
            | _, [] -> 1
            | hd0 :: tl0, hd1 :: tl1 -> (
                cmp (hd0, hd1) |> function 0 -> loop (tl0, tl1) | o -> o)
          in
          loop (l0, l1)
    in
    cmp (v0, v1) < 0

  let parse input =
    let parse_chunks = function
      | [ left; right ] -> (parse_packet left, parse_packet right)
      | _ -> failwith "invalid input"
    in
    Stdio.In_channel.input_lines input
    |> List.chunks_of ~length:3
    |> List.map ~f:(Fn.flip List.take 2)
    |> List.map ~f:parse_chunks

  let part1 pairs =
    let f i (l, r) = if l < r then Some (i + 1) else None in
    pairs |> List.filter_mapi ~f
    |> List.sum (module Int) ~f:Fn.id
    |> Solution.answer_of_int

  let part2 pairs =
    let packets =
      let to_list pair = [ fst pair; snd pair ] in
      pairs |> List.map ~f:to_list |> List.concat
    in
    let f i p = i + 1 + List.count packets ~f:(Fn.flip ( < ) p) in
    [ "[[2]]"; "[[6]]" ] |> List.map ~f:parse_packet |> List.mapi ~f
    |> List.fold ~init:1 ~f:( * ) |> Solution.answer_of_int
end
