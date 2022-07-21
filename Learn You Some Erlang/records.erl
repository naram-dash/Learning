-module(records).

-compile(export_all).

-include("records.hrl").

-record(robot, {name, type = industrial, hobbies, details = []}).

repairman(Rob) ->
  Details = Rob#robot.details,
  NewRob = Rob#robot{details = ["Repaired by repairman" | Details]},
  {repaired, NewRob}.

included() -> #included{some_field="Some value"}.