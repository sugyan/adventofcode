open Base
open Solution

module Solution : Solve = struct
  type t = ((int * int) * (int * int)) list

  let max_coordinate =
    Sys.getenv "MAX_COORDINATE" |> function
    | Some s -> Int.of_string s
    | None -> 4_000_000

  let distance (x1, y1) (x2, y2) = abs (x1 - x2) + abs (y1 - y2)

  let ranges y reports =
    let rec loop = function
      | ([] | [ _ ]) as acc -> acc
      | (xmin0, xmax0) :: (xmin1, xmax1) :: tl ->
          if xmin1 > xmax0 + 1 then (xmin0, xmax0) :: loop tl
          else loop ((xmin0, max xmax0 xmax1) :: tl)
    in
    List.filter_map reports ~f:(fun ((sx, sy), (bx, by)) ->
        let d = distance (sx, sy) (bx, by) - abs (y - sy) in
        if d >= 0 then Some (sx - d, sx + d) else None)
    |> List.sort ~compare:Poly.compare
    |> loop

  let parse input =
    let parse_line line =
      Caml.Scanf.sscanf line
        "Sensor at x=%d, y=%d: closest beacon is at x=%d, y=%d"
        (fun sx sy bx by -> ((sx, sy), (bx, by)))
    in
    Stdio.In_channel.input_lines input |> List.map ~f:parse_line

  let part1 reports =
    let xs =
      List.filter_map reports ~f:(fun (_, (x, y)) ->
          if y = max_coordinate / 2 then Some x else None)
      |> List.dedup_and_sort ~compare
      |> List.length
    in
    reports
    |> ranges (max_coordinate / 2)
    |> List.sum (module Int) ~f:(fun (xmin, xmax) -> xmax - xmin + 1)
    |> Fn.flip ( - ) xs |> answer_of_int

  let part2 reports =
    let ds =
      List.map reports ~f:(fun ((sx, sy), b) -> (sx, sy, distance (sx, sy) b))
    in
    List.cartesian_product
      (List.fold ds ~init:[ 0 ] ~f:(fun acc (x, y, d) ->
           (y - x + (d + 1)) :: (y - x - (d + 1)) :: acc))
      (List.fold ds ~init:[ max_coordinate ] ~f:(fun acc (x, y, d) ->
           (y + x + (d + 1)) :: (y + x - (d + 1)) :: acc))
    |> List.map ~f:(fun (b0, b1) -> (b0 + b1) / 2)
    |> List.filter ~f:(fun y -> y >= 0 && y <= max_coordinate)
    |> List.dedup_and_sort ~compare
    |> List.find_map_exn ~f:(fun y ->
           ranges y reports |> function
           | [ (_, xmax); _ ] -> Some (((xmax + 1) * 4_000_000) + y)
           | _ -> None)
    |> answer_of_int
end
