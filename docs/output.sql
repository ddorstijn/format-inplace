insert	
into	notification_relation
	(	notification_id
	,	relation_id
	,	active_yn
	)	
select	par.notification_id "NOTIFICATION_ID"
,	r.relation_id "RELATION_ID"
,	'Y' "ACTIVE_YN"
from	import_not_relations_of_filter par
,	relation r
where	
	(	par.relation_id is null
	or	r.relation_id = par.relation_id
	)	
and	
	(	not exists
		(	select	''
			from	notification_filter nf
			where	nf.notification_id = par.notification_id
			and	nf.filter_type in
				(	'RN'
				,	'PC'
				,	'PR'
				)	
		)	
	or	exists
		(	select	''
			from	notification_filter nf
			where	nf.notification_id = par.notification_id
			and	
				(	
					(	nf.filter_type = 'RN'
					and	r.relation_id = nf.relation_id
					)	
				or	
					(	nf.filter_type = 'PC'
					and	r.postal_code = nf.postal_code
					)	
				or	
					(	nf.filter_type = 'PR'
					and	to_integer
						(	substr
							(	r.postal_code
							,	1
							,	4
							)	
						)	
						between	nf.postal_code_start
						and	nf.postal_code_end
					)	
				)	
		)	
	)	
and	
	(	not exists
		(	select	''
			from	notification_filter nf
			where	nf.notification_id = par.notification_id
			and	nf.filter_type = 'PY'
		)	
	or	exists
		(	select	''
			from	notification_filter nf
			,	recipient rt
			,	delivery_address da
			where	rt.recipient_id = r.relation_id
			and	nf.notification_id = par.notification_id
			and	da.poultry_on_premises = nf.has_poultry
		)	
	)	
and	not exists
	(	select	''
		from	notification_relation nr
		where	nr.notification_id = par.notification_id
		and	nr.relation_id = r.relation_id
	)	
;	
	
update	notification_filter nf
set	nf.active_yn =
	(	select	par.notification_id
		from	import_not_relations_of_filter par
	)	
where	nf.filter_type = 'PY'
;	
	
delete	
from	notification_filter
where	notification_id = par.notification_id
;	
	
select	batchrunner.xml_export
	(	r.id
	,	r.name
	)	
from	relation r
;	
	
invoke	batchrunner.xml_import
	(	'import_not_relations_of_filter'
	,	case	
		when	exists
			(	select	''
				from	notification_filter nf
				where	nf.filter_type = 'PY'
			)	then 1
		else	0
		end	
	)	
from	table_a a
	join	relation r
	on	r.name = a.x
;	
	
	
;	
	
	
