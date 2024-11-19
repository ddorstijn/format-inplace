/* This is a comment */	
insert	
into	 notification_relation
(	 notification_id
,	 relation_id
,	 active_yn
)	
select	 par.notification_id"NOTIFICATION_ID"
,	 r.relation_id"RELATION_ID"
,	 'Y'"ACTIVE_YN"
from	 import_not_relations_of_filterpar
,	 relationr
where	
(	 par.relation_idisnull
or	 r.relation_id=par.relation_id
)	
and	
(	 notexists
(	
select	 ''
from	 notification_filternf
where	 nf.notification_id=par.notification_id
and	 nf.filter_typein
(	 'RN'
,	 'PC'
,	 'PR'
)	
)	
or	 exists
(	
select	 ''
from	 notification_filternf
where	 nf.notification_id=par.notification_id
and	
(	
(	 nf.filter_type='RN'
and	 r.relation_id=nf.relation_id
)	
or	
(	 nf.filter_type='PC'
and	 r.postal_code=nf.postal_code
)	
or	
(	 nf.filter_type='PR'
and	 to_integer
(	 substr
(	 r.postal_code
,	 1
,	 4
)	
)	 betweennf.postal_code_start
and	 nf.postal_code_end
)	
)	
)	
)	
and	
(	 notexists
(	
select	 ''
from	 notification_filternf
where	 nf.notification_id=par.notification_id
and	 nf.filter_type='PY'
)	
or	 exists
(	
select	 ''
from	 notification_filternf
,	 recipientrt
,	 delivery_addressda
where	 rt.recipient_id=r.relation_id
and	 nf.notification_id=par.notification_id
and	 da.poultry_on_premises=nf.has_poultry
)	
or	
(	 'all'=nf.has_poultry
)	
)	
and	 notexists
(	
select	 ''
from	 notification_relationnr
where	 nr.notification_id=par.notification_id
and	 nr.relation_id=r.relation_id
)	
;	
update	 notification_filternf
set	 notification_id=
(	
select	 par.notification_id
from	 import_not_relations_of_filterpar
)	
where	 nf.filter_type='PY'
;	
delete	
from	 notification_filter
where	 notification_id=par.notification_id
;	
	 nr 
		where 
			nr.notification_id = par.notification_id 
		and
			nr.relation_id = r.relation_id
	)
;

update	notification_filter nf
set	notification_id = (
		select
			par.notification_id
		from
			import_not_relations_of_filter par
	)
where	nf.filter_type = 'PY'
;

delete
from	notification_filter
where	notification_id = par.notification_id
;

ar.notification_id

;

