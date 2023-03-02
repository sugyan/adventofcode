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
    let rec loop acc = function
      | [] -> acc
      | (xmin, xmax) :: tl ->
          (match acc with
          | [] -> [ (xmin, xmax) ]
          | (xmin', xmax') :: tl' ->
              if xmin > xmax' + 1 then (xmin, xmax) :: acc
              else (xmin', max xmax xmax') :: tl')
          |> Fn.flip loop tl
    in
    List.filter_map reports ~f:(fun ((sx, sy), (bx, by)) ->
        let d = distance (sx, sy) (bx, by) - abs (y - sy) in
        if d >= 0 then Some (sx - d, sx + d) else None)
    |> List.sort ~compare:Poly.compare
    |> loop []

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
    List.range 0 max_coordinate
    |> List.find_map_exn ~f:(fun y ->
           ranges y reports |> function
           | [ (xmin, _); _ ] -> Some (((xmin - 1) * 4_000_000) + y)
           | _ -> None)
    |> answer_of_int
end
