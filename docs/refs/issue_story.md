# Agile Story

in Jira

```xml
    <entity entity-name="Issue" table-name="jiraissue" package-name="">
        <field name="id" type="numeric"/>

        <field name="key" col-name="pkey" type="long-varchar"/>

        <field name="number" col-name="issuenum" type="numeric"/>

        <field name="project" type="numeric"/>

        <field name="reporter" type="long-varchar"/>
        <field name="assignee" type="long-varchar"/>
        <field name="creator" type="long-varchar"/>

        <field name="type" col-name="issuetype" type="long-varchar"/>
        <field name="summary" type="long-varchar"/>
        <field name="description" type="extremely-long"/>
        <field name="environment" type="extremely-long"/>
        <field name="priority" type="long-varchar"/>
        <field name="resolution" type="long-varchar"/>
        <field name="status" col-name="issuestatus" type="long-varchar"/>
        <field name="created" type="date-time"/>
        <field name="updated" type="date-time"/>
        <field name="duedate" type="date-time"/>
        <field name="resolutiondate" type="date-time"/>

        <field name="votes" type="numeric"/>
        <field name="watches" type="numeric"/>
        <field name="timeoriginalestimate" type="numeric"/>
        <field name="timeestimate" type="numeric"/>
        <field name="timespent" type="numeric"/>

        <field name="workflowId" type="numeric"/>
        <field name="security" type="numeric"/>

        <!-- deprecated fields - do not use -->
        <field name="fixfor" type="numeric"/>
        <field name="component" type="numeric"/>

        <field name="archived" type="indicator"/>
        <field name="archivedby" type="long-varchar"/>
        <field name="archiveddate" type="date-time"/>

        <prim-key field="id"/>

        <relation type="one" title="Parent" rel-entity-name="Project">
            <key-map field-name="project" rel-field-name="id"/>
        </relation>
        <relation type="many" title="Child" rel-entity-name="Action">
            <key-map field-name="id" rel-field-name="issue"/>
        </relation>
        <relation type="many" title="Child" rel-entity-name="Worklog">
            <key-map field-name="id" rel-field-name="issue"/>
        </relation>
        <relation type="many" title="Child" rel-entity-name="FileAttachment">
            <key-map field-name="id" rel-field-name="issue"/>
        </relation>
        <relation type="many" title="Source" rel-entity-name="IssueLink">
            <key-map field-name="id" rel-field-name="source"/>
        </relation>
        <relation type="many" title="Destination" rel-entity-name="IssueLink">
            <key-map field-name="id" rel-field-name="destination"/>
        </relation>
        <relation type="many" title="Child" rel-entity-name="RemoteIssueLink">
            <key-map field-name="id" rel-field-name="issueid"/>
        </relation>
        <relation type="many" title="Child" rel-entity-name="CustomFieldValue">
            <key-map field-name="id" rel-field-name="issue"/>
        </relation>
        <relation type="many" title="Child" rel-entity-name="ChangeGroup">
            <key-map field-name="id" rel-field-name="issue"/>
        </relation>
        <relation type="many" title="Child" rel-entity-name="Label">
            <key-map field-name="id" rel-field-name="issue"/>
        </relation>
        <relation type="one" title="Parent" rel-entity-name="SchemeIssueSecurityLevels">
            <key-map field-name="security" rel-field-name="id"/>
        </relation>

        <relation type="one" title="Related" rel-entity-name="OSWorkflowEntry">
            <key-map field-name="workflowId" rel-field-name="id"/>
        </relation>
        <relation type="one" title="Related" rel-entity-name="OSCurrentStep">
            <key-map field-name="workflowId" rel-field-name="entryId"/>
        </relation>
        <relation type="one" title="Child" rel-entity-name="TrackbackPing">
            <key-map field-name="id" rel-field-name="issue"/>
        </relation>

        <!-- issue_proj_num should be unique, but cannot as long as we support
             upgrade from 6.0.x until than it is made unique by UpgradeTask_Build6132 -->
        <index name="issue_proj_num">
            <index-field name="number"/>
            <index-field name="project"/>
        </index>
        <index name="issue_proj_status">
            <index-field name="project"/>
            <index-field name="status"/>
        </index>
        <index name="issue_created">
            <index-field name="created"/>
        </index>
        <index name="issue_updated">
            <index-field name="updated"/>
        </index>
        <index name="issue_duedate">
            <index-field name="duedate"/>
        </index>
        <index name="issue_resolutiondate">
            <index-field name="resolutiondate"/>
        </index>
        <index name="issue_assignee">
            <index-field name="assignee"/>
        </index>
        <index name="issue_reporter">
            <index-field name="reporter"/>
        </index>
        <index name="issue_workflow">
            <index-field name="workflowId"/>
        </index>
        <index name="issue_votes">
            <index-field name="votes"/>
        </index>
        <index name="issue_watches">
            <index-field name="watches"/>
        </index>
    </entity>
```

Mingle

```ruby
  create_table "cards", force: :cascade do |t|
    t.integer  "project_id",                                      null: false
    t.integer  "number",                                          null: false
    t.text     "description"
    t.datetime "created_at",                                      null: false
    t.datetime "updated_at",                                      null: false
    t.integer  "version"
    t.string   "card_type_name",      limit: 255,                 null: false
    t.boolean  "has_macros",                      default: false
    t.decimal  "project_card_rank"
    t.integer  "caching_stamp",                   default: 0,     null: false
    t.string   "name",                limit: 255,                 null: false
    t.integer  "created_by_user_id",                              null: false
    t.integer  "modified_by_user_id",                             null: false
    t.boolean  "redcloth"
    t.index ["number"], name: "index_cards_on_number", using: :btree
    t.index ["project_id"], name: "index_cards_on_project_id", using: :btree
  end
```

Zentao

```sql
CREATE TABLE IF NOT EXISTS `zt_bug` (
  `id` mediumint(8) NOT NULL auto_increment,
  `project` mediumint(8) unsigned NOT NULL,
  `product` mediumint(8) unsigned NOT NULL default '0',
  `branch` mediumint(8) unsigned NOT NULL default '0',
  `module` mediumint(8) unsigned NOT NULL default '0',
  `execution` mediumint(8) unsigned NOT NULL default '0',
  `plan` mediumint(8) unsigned NOT NULL default '0',
  `story` mediumint(8) unsigned NOT NULL default '0',
  `storyVersion` smallint(6) NOT NULL default '1',
  `task` mediumint(8) unsigned NOT NULL default '0',
  `toTask` mediumint(8) unsigned NOT NULL default '0',
  `toStory` mediumint(8) NOT NULL default '0',
  `title` varchar(255) NOT NULL,
  `keywords` varchar(255) NOT NULL,
  `severity` tinyint(4) NOT NULL default '0',
  `pri` tinyint(3) unsigned NOT NULL,
  `type` varchar(30) NOT NULL default '',
  `os` varchar(30) NOT NULL default '',
  `browser` varchar(30) NOT NULL default '',
  `hardware` varchar(30) NOT NULL,
  `found` varchar(30) NOT NULL default '',
  `steps` text NOT NULL,
  `status` enum('active','resolved','closed') NOT NULL default 'active',
  `subStatus` varchar(30) NOT NULL default '',
  `color` char(7) NOT NULL,
  `confirmed` tinyint(1) NOT NULL default '0',
  `activatedCount` smallint(6) NOT NULL,
  `activatedDate` datetime NOT NULL,
  `mailto` text,
  `openedBy` varchar(30) NOT NULL default '',
  `openedDate` datetime NOT NULL,
  `openedBuild` varchar(255) NOT NULL,
  `assignedTo` varchar(30) NOT NULL default '',
  `assignedDate` datetime NOT NULL,
  `deadline` date NOT NULL,
  `resolvedBy` varchar(30) NOT NULL default '',
  `resolution` varchar(30) NOT NULL default '',
  `resolvedBuild` varchar(30) NOT NULL default '',
  `resolvedDate` datetime NOT NULL,
  `closedBy` varchar(30) NOT NULL default '',
  `closedDate` datetime NOT NULL,
  `duplicateBug` mediumint(8) unsigned NOT NULL,
  `linkBug` varchar(255) NOT NULL,
  `case` mediumint(8) unsigned NOT NULL,
  `caseVersion` smallint(6) NOT NULL DEFAULT '1',
  `result` mediumint(8) unsigned NOT NULL,
  `repo` mediumint(8) unsigned NOT NULL,
  `entry` varchar(255) NOT NULL,
  `lines` varchar(10) NOT NULL,
  `v1` varchar(40) NOT NULL,
  `v2` varchar(40) NOT NULL,
  `repoType` varchar(30) NOT NULL DEFAULT '',
  `testtask` mediumint(8) unsigned NOT NULL,
  `lastEditedBy` varchar(30) NOT NULL default '',
  `lastEditedDate` datetime NOT NULL,
  `deleted` enum('0','1') NOT NULL default '0',
  PRIMARY KEY (`id`),
  KEY `product` (`product`),
  KEY `execution` (`execution`),
  KEY `status` (`status`),
  KEY `plan` (`plan`),
  KEY `story` (`story`),
  KEY `case` (`case`),
  KEY `toStory` (`toStory`),
  KEY `result` (`result`),
  KEY `assignedTo` (`assignedTo`)
) ENGINE=MyISAM DEFAULT CHARSET=utf8
```