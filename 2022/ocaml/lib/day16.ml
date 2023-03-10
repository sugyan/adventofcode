open Base
open Solution

module Solution : Solve = struct
  type t = int -> (int * int) list
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
          List.mapi valves ~f:(fun j { label = dst; _ } -> (dst, m.(i).(j)))
          |> Hashtbl.of_alist_exn (module String)
          |> fun tbl -> (src, tbl))
      |> Hashtbl.of_alist_exn (module String)
    in
    let targets =
      List.filter_map valves ~f:(fun { label; flow_rate; _ } ->
          if flow_rate > 0 then Some (label, flow_rate) else None)
    in
    fun minutes ->
      let rec totals ~src ~opened ~minutes ~total acc =
        let dist = Hashtbl.find_exn dists src in
        targets
        |> List.filter_mapi ~f:(fun i target ->
               let o = 1 lsl i in
               if o land opened = 0 then Some (target, o) else None)
        |> List.fold ~init:((total, opened) :: acc)
             ~f:(fun acc ((dst, rate), o) ->
               let remain = minutes - 1 - Hashtbl.find_exn dist dst in
               if remain > 0 then
                 totals ~src:dst ~opened:(o lor opened) ~minutes:remain
                   ~total:(total + (remain * rate))
                   acc
               else acc)
      in
      totals ~src:"AA" ~opened:0 ~minutes ~total:0 []

  let part1 max_totals =
    max_totals 30 |> List.map ~f:fst |> List.fold ~init:0 ~f:max
    |> answer_of_int

  let part2 max_totals =
    let sorted =
      max_totals 26
      |> Hashtbl.group (module Int) ~get_key:snd ~get_data:fst ~combine:max
      |> Hashtbl.to_alist
      |> List.sort ~compare:(fun (_, t0) (_, t1) -> compare t1 t0)
    in
    List.fold_until sorted ~init:0
      ~f:(fun best (o0, t0) ->
        if t0 * 2 < best then Stop best
        else
          Continue
            (List.filter_map sorted ~f:(fun (o1, t1) ->
                 if o0 land o1 = 0 then Some (t0 + t1) else None)
            |> List.fold ~init:best ~f:max))
      ~finish:Fn.id
    |> answer_of_int
end
