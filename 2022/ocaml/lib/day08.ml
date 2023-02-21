open Base

module Solution : Solution.Solve = struct
  type t = bool list list list

  let parse input =
    let grid =
      let to_int_array s = String.to_array s |> Array.map ~f:Char.to_int in
      Stdio.In_channel.input_lines input
      |> List.map ~f:to_int_array |> Array.of_list
    in
    let r, c = (Array.length grid, Array.length grid.(0)) in
    let f (i, j) =
      let collect_trees (di, dj) =
        let rec loop acc i j =
          if i < 0 || i >= r || j < 0 || j >= c then acc
          else loop (grid.(i).(j) :: acc) (i + di) (j + dj)
        in
        loop [] (i + di) (j + dj) |> List.rev
      in
      [ (-1, 0); (1, 0); (0, -1); (0, 1) ]
      |> List.map ~f:collect_trees
      |> List.map ~f:(List.map ~f:(Fn.flip ( < ) grid.(i).(j)))
    in
    List.cartesian_product (List.range 0 r) (List.range 0 c) |> List.map ~f

  let part1 lowers =
    let visible l =
      List.map l ~f:(List.for_all ~f:Fn.id) |> List.fold ~init:false ~f:( || )
    in
    lowers |> List.count ~f:visible |> Solution.answer_of_int

  let part2 lowers =
    let scenic_score l =
      let distance lower =
        let f _ = Fn.id |> Fn.non in
        List.findi lower ~f |> function
        | Some (i, _) -> i + 1
        | None -> List.length lower
      in
      List.map l ~f:distance |> List.fold ~init:1 ~f:( * )
    in
    lowers |> List.map ~f:scenic_score
    |> List.fold ~init:0 ~f:Int.max
    |> Solution.answer_of_int
end
