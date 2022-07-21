-module(records).

-compile(export_all).

% after = is default value
-record(robot, {name, type = industrial, hobbies, details = []}).

first_robot() ->
  #robot{type = handmade,
         name = "Mechatron",
         details = ["Moved by a small man inside"]}.

car_factory(CorpName) ->
  #robot{name = CorpName, hobbies = "building cars"}.

-record(user, {id, name, group, age}).

admin_panel(#user{name=Name, group=admin}) -> 
  Name ++ " is allowed!";
admin_panel(#user{name=Name}) ->
  Name ++ " is not allowed!".

%% can extend user without problem
adult_section(U = #user{}) when U#user.age >= 18 ->
  %% Show stuff that can't be written in such a text
  allowed;
adult_section(_) ->
  %% redirect to sesame street site
  forbidden.