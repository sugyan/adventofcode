open Base
open Solution

module Solution : Solve = struct
  type t = bool list list list

  let parse input =
    let grid =
      Stdio.In_channel.input_lines input
      |> List.map ~f:(fun s -> String.to_array s |> Array.map ~f:Char.to_int)
      |> Array.of_list
    in
    let r, c = (Array.length grid, Array.length grid.(0)) in
    let lowers (i, j) =
      let trees (di, dj) =
        let rec loop acc i j =
          if i < 0 || i >= r || j < 0 || j >= c then acc
          else loop (grid.(i).(j) :: acc) (i + di) (j + dj)
        in
        loop [] (i + di) (j + dj) |> List.rev
      in
      [ (-1, 0); (1, 0); (0, -1); (0, 1) ]
      |> List.map ~f:trees
      |> List.map ~f:(List.map ~f:(Fn.flip ( < ) grid.(i).(j)))
    in
    List.cartesian_product (List.range 0 r) (List.range 0 c)
    |> List.map ~f:lowers

  let part1 lowers =
    let visible l =
      List.map l ~f:(List.for_all ~f:Fn.id) |> List.fold ~init:false ~f:( || )
    in
    lowers |> List.count ~f:visible |> answer_of_int

  let part2 lowers =
    let scenic_score l =
      let distance lower =
        List.findi lower ~f:(Fn.non Fn.id |> Fn.const) |> function
        | Some (i, _) -> i + 1
        | None -> List.length lower
      in
      List.map l ~f:distance |> List.fold ~init:1 ~f:( * )
    in
    lowers |> List.map ~f:scenic_score |> List.fold ~init:0 ~f:max
    |> answer_of_int
end
