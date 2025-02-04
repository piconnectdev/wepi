drop view community_view;
create view community_view as 
with all_community as
(
  select *,
  (select name from user_ u where c.creator_id = u.id) as creator_name,
  (select name from category ct where c.category_id = ct.id) as category_name,
  (select count(*) from community_follower cf where cf.community_id = c.id) as number_of_subscribers,
  (select count(*) from post p where p.community_id = c.id) as number_of_posts,
  (select count(*) from comment co, post p where c.id = p.community_id and p.id = co.post_id) as number_of_comments,
  hot_rank((select count(*) from community_follower cf where cf.community_id = c.id), c.published) as hot_rank
  from community c
)

select
ac.*,
u.id as user_id,
(select cf.id from community_follower cf where u.id = cf.user_id and ac.id = cf.community_id) is not null as subscribed
from user_ u
cross join all_community ac

union all

select 
ac.*,
null as user_id,
null as subscribed
from all_community ac
;
