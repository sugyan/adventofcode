open Base
open Solution

module Solution : Solve = struct
  type blueprint = { costs : int list list; maxs : int list }
  type t = blueprint list

  let max_geodes minutes blueprint =
    let rec dfs robots resources minutes =
      List.zip_exn
        (List.zip_exn blueprint.costs blueprint.maxs)
        (List.zip_exn robots resources)
      |> List.mapi ~f:(fun i x -> (i, x))
      |> List.filter ~f:(fun (i, ((costs, max), (robot, resource))) ->
             List.zip_exn costs robots
             |> List.for_all ~f:(fun (cost, robot) -> cost = 0 || robot > 0)
             && (i = 3 || resource < (max - robot) * minutes))
      |> List.filter_map ~f:(fun (i, ((costs, _), _)) ->
             let wait =
               List.map3_exn robots resources costs
                 ~f:(fun robot resource cost ->
                   if robot = 0 || resource >= cost then 0
                   else ((cost - resource - 1) / robot) + 1)
               |> List.fold ~init:0 ~f:max
             in
             if wait < minutes then Some (i, wait, costs) else None)
      |> List.map ~f:(fun (i, wait, costs) ->
             dfs
               (List.mapi robots ~f:(fun j robot ->
                    if j = i then robot + 1 else robot))
               (List.map3_exn robots resources costs
                  ~f:(fun robot resource cost ->
                    resource + ((wait + 1) * robot) - cost))
               (minutes - wait - 1))
      |> List.fold
           ~init:(List.nth_exn resources 3 + (List.nth_exn robots 3 * minutes))
           ~f:max
    in
    dfs [ 1; 0; 0; 0 ] [ 0; 0; 0; 0 ] minutes

  let parse input =
    let parse_line line =
      Caml.Scanf.sscanf line
        "Blueprint %d: Each ore robot costs %d ore. Each clay robot costs %d \
         ore. Each obsidian robot costs %d ore and %d clay. Each geode robot \
         costs %d ore and %d obsidian."
        (fun _ ore_ore cla_ore obs_ore obs_cla geo_ore geo_obs ->
          {
            costs =
              [
                [ ore_ore; 0; 0; 0 ];
                [ cla_ore; 0; 0; 0 ];
                [ obs_ore; obs_cla; 0; 0 ];
                [ geo_ore; 0; geo_obs; 0 ];
              ];
            maxs =
              [
                ore_ore |> max cla_ore |> max obs_ore |> max geo_ore;
                obs_cla;
                geo_obs;
                0;
              ];
          })
    in
    Stdio.In_channel.input_lines input |> List.map ~f:parse_line

  let part1 blueprints =
    blueprints
    |> List.mapi ~f:(fun i blueprint -> (i + 1) * max_geodes 24 blueprint)
    |> List.sum (module Int) ~f:Fn.id
    |> answer_of_int

  let part2 blueprint =
    List.take blueprint 3
    |> List.map ~f:(max_geodes 32)
    |> List.fold ~init:1 ~f:( * ) |> answer_of_int
end
