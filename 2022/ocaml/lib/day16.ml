open Base
open Solution

module Solution : Solve = struct
  type t = int -> int list
  type valve = { label : string; flow_rate : int; tunnels : string list }

  let parse input =
    let parse_line line =
      let valve, tunnels = String.lsplit2_exn line ~on:';' in
      let label, flow_rate =
        Caml.Scanf.sscanf valve "Valve %s has flow rate=%d" (fun l r -> (l, r))
      in
      let tunnels =
        String.split ~on:' ' tunnels
        |> Fn.flip List.drop 5
        |> List.map ~f:(String.chop_suffix_if_exists ~suffix:",")
      in
      { label; flow_rate; tunnels }
    in
    let valves = Stdio.In_channel.input_lines input |> List.map ~f:parse_line in
    let dists =
      let length = List.length valves in
      let tbl =
        List.mapi valves ~f:(fun i { label; _ } -> (label, i))
        |> Hashtbl.of_alist_exn (module String)
      in
      let m = Array.make_matrix ~dimx:length ~dimy:length length in
      List.iteri valves ~f:(fun i { tunnels; _ } ->
          m.(i).(i) <- 0;
          List.map tunnels ~f:(Hashtbl.find_exn tbl)
          |> List.iter ~f:(fun j -> m.(i).(j) <- 1));
      for k = 0 to length - 1 do
        for i = 0 to length - 1 do
          for j = 0 to length - 1 do
            m.(i).(j) <- min m.(i).(j) (m.(i).(k) + m.(k).(j))
          done
        done
      done;
      List.mapi valves ~f:(fun i { label = src; _ } ->
          ( src,
            List.mapi valves ~f:(fun j { label = dst; _ } -> (dst, m.(i).(j)))
            |> Hashtbl.of_alist_exn (module String) ))
      |> Hashtbl.of_alist_exn (module String)
    in
    let targets =
      List.filter_map valves ~f:(fun { label; flow_rate; _ } ->
          if flow_rate > 0 then Some (label, flow_rate) else None)
    in
    fun minutes ->
      let rec totals src opened minutes total =
        let dist dst =
          Hashtbl.find_exn dists src |> Fn.flip Hashtbl.find_exn dst
        in
        total
        :: (targets
           |> List.filter_map ~f:(fun (dst, rate) ->
                  if Set.mem opened dst then None
                  else
                    let remain = minutes - 1 - dist dst in
                    if remain <= 0 then None
                    else
                      Some
                        (totals dst
                           (Set.union opened
                              (Set.of_list (module String) [ dst ]))
                           remain
                           (total + (remain * rate))))
           |> List.concat)
      in
      totals "AA" (Set.empty (module String)) minutes 0

  let part1 max_totals =
    max_totals 30 |> List.fold ~init:0 ~f:max |> answer_of_int

  let part2 _ = failwith "Not implemented"
end
