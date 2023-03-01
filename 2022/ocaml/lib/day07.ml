open Base
open Solution

module Solution : Solve = struct
  type t = int list

  let parse input =
    let table = Hashtbl.create (module String) in
    let f path = function
      | "$ cd", ".." -> List.tl_exn path
      | "$ cd", dir -> dir :: path
      | ("dir" | "$"), _ -> path
      | s, _ ->
          let update n =
            Hashtbl.update table ~f:(function Some sum -> sum + n | None -> n)
          in
          List.rev path
          |> Utils.scan ~init:"" ~f:(fun acc dir -> dir ^ "/" ^ acc)
          |> List.iter ~f:(update (Int.of_string s));
          path
    in
    Stdio.In_channel.input_lines input
    |> List.map ~f:(String.rsplit2_exn ~on:' ')
    |> List.fold ~init:[] ~f |> ignore;
    Hashtbl.data table

  let part1 total_scores =
    total_scores
    |> List.filter ~f:(Fn.flip ( <= ) 100_000)
    |> List.sum (module Int) ~f:Fn.id
    |> answer_of_int

  let part2 total_sizes =
    let max = List.fold total_sizes ~init:0 ~f:max in
    total_sizes
    |> List.filter ~f:(fun size -> max - size < 40_000_000)
    |> List.min_elt ~compare |> Option.value_exn |> answer_of_int
end
