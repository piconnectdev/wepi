-- Add the column
alter table post add column stickied boolean default false not null;

-- Add the mod table
create table mod_sticky_post (
  id uuid NOT NULL DEFAULT next_uuid() primary key,
  mod_user_id uuid references user_ on update cascade on delete cascade not null,
  post_id uuid references post on update cascade on delete cascade not null,
  stickied boolean default true,
  when_ timestamp not null default now()
);

-- Add mod view
create view mod_sticky_post_view as 
select msp.*,
(select name from user_ u where msp.mod_user_id = u.id) as mod_user_name,
(select name from post p where msp.post_id = p.id) as post_name,
(select c.id from post p, community c where msp.post_id = p.id and p.community_id = c.id) as community_id,
(select c.name from post p, community c where msp.post_id = p.id and p.community_id = c.id) as community_name
from mod_sticky_post msp;

-- Recreate the view
drop view post_view;
create view post_view as
with all_post as
(
  select        
  p.*,
  (select u.banned from user_ u where p.creator_id = u.id) as banned,
  (select cb.id from community_user_ban cb where p.creator_id = cb.user_id and p.community_id = cb.community_id) is not null as banned_from_community,
  (select name from user_ where p.creator_id = user_.id) as creator_name,
  (select name from community where p.community_id = community.id) as community_name,
  (select removed from community c where p.community_id = c.id) as community_removed,
  (select deleted from community c where p.community_id = c.id) as community_deleted,
  (select nsfw from community c where p.community_id = c.id) as community_nsfw,
  (select count(*) from comment where comment.post_id = p.id) as number_of_comments,
  coalesce(sum(pl.score), 0) as score,
  count (case when pl.score = 1 then 1 else null end) as upvotes,
  count (case when pl.score = -1 then 1 else null end) as downvotes,
  hot_rank(coalesce(sum(pl.score) , 0), p.published) as hot_rank
  from post p
  left join post_like pl on p.id = pl.post_id
  group by p.id
)

select
ap.*,
u.id as user_id,
coalesce(pl.score, 0) as my_vote,
(select cf.id from community_follower cf where u.id = cf.user_id and cf.community_id = ap.community_id) is not null as subscribed,
(select pr.id from post_read pr where u.id = pr.user_id and pr.post_id = ap.id) is not null as read,
(select ps.id from post_saved ps where u.id = ps.user_id and ps.post_id = ap.id) is not null as saved
from user_ u
cross join all_post ap
left join post_like pl on u.id = pl.user_id and ap.id = pl.post_id

union all

select 
ap.*,
null as user_id,
null as my_vote,
null as subscribed,
null as read,
null as saved
from all_post ap
;
