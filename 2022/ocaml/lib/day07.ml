open Base

module Solution : Solution.Solve = struct
  type t = int list

  let parse input =
    let table = Hashtbl.create (module String) in
    let folding_update size acc dir =
      let id = dir ^ "/" ^ acc in
      Hashtbl.update table id ~f:(function
        | None -> size
        | Some sum -> sum + size);
      id
    in
    let f path = function
      | "$ cd", ".." -> List.tl_exn path
      | "$ cd", dir -> dir :: path
      | "dir", _ | "$", "ls" -> path
      | s, _ ->
          List.fold_right path ~init:"" ~f:(folding_update (Int.of_string s))
          |> ignore;
          path
    in
    Stdio.In_channel.input_lines input
    |> List.map ~f:(String.rsplit2_exn ~on:' ')
    |> List.fold ~init:[] ~f |> ignore;
    Hashtbl.data table

  let part1 total_scores =
    total_scores
    |> List.filter ~f:(Fn.flip Int.( <= ) 100_000)
    |> List.sum (module Int) ~f:Fn.id
    |> Solution.answer_of_integer

  let part2 total_sizes =
    let max = List.fold total_sizes ~init:0 ~f:Int.max in
    let free_up_enough size = max - size < 40_000_000 in
    total_sizes
    |> List.filter ~f:free_up_enough
    |> List.min_elt ~compare:Int.compare
    |> Option.value_exn |> Solution.answer_of_integer
end
