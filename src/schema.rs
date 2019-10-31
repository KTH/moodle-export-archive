table! {
    mdl_assign (id) {
        id -> Int8,
        course -> Int8,
        name -> Varchar,
        intro -> Text,
        introformat -> Int2,
        alwaysshowdescription -> Int2,
        nosubmissions -> Int2,
        submissiondrafts -> Int2,
        sendnotifications -> Int2,
        sendlatenotifications -> Int2,
        duedate -> Int8,
        allowsubmissionsfromdate -> Int8,
        grade -> Int8,
        timemodified -> Int8,
        requiresubmissionstatement -> Int2,
        completionsubmit -> Int2,
        cutoffdate -> Int8,
        teamsubmission -> Int2,
        requireallteammemberssubmit -> Int2,
        teamsubmissiongroupingid -> Int8,
        blindmarking -> Int2,
        revealidentities -> Int2,
    }
}

table! {
    mdl_assignfeedback_comments (id) {
        id -> Int8,
        assignment -> Int8,
        grade -> Int8,
        commenttext -> Nullable<Text>,
        commentformat -> Int2,
    }
}

table! {
    mdl_assignfeedback_file (id) {
        id -> Int8,
        assignment -> Int8,
        grade -> Int8,
        numfiles -> Int8,
    }
}

table! {
    mdl_assign_grades (id) {
        id -> Int8,
        assignment -> Int8,
        userid -> Int8,
        timecreated -> Int8,
        timemodified -> Int8,
        grader -> Int8,
        grade -> Nullable<Numeric>,
        locked -> Int2,
        mailed -> Int2,
        extensionduedate -> Int8,
    }
}

/* Note: Empty table
table! {
    mdl_assignment (id) {
        id -> Int8,
        course -> Int8,
        name -> Varchar,
        intro -> Text,
        introformat -> Int2,
        assignmenttype -> Varchar,
        resubmit -> Int2,
        preventlate -> Int2,
        emailteachers -> Int2,
        var1 -> Nullable<Int8>,
        var2 -> Nullable<Int8>,
        var3 -> Nullable<Int8>,
        var4 -> Nullable<Int8>,
        var5 -> Nullable<Int8>,
        maxbytes -> Int8,
        timedue -> Int8,
        timeavailable -> Int8,
        grade -> Int8,
        timemodified -> Int8,
    }
}
 */

/* Empty table
table! {
    mdl_assignment_submissions (id) {
        id -> Int8,
        assignment -> Int8,
        userid -> Int8,
        timecreated -> Int8,
        timemodified -> Int8,
        numfiles -> Int8,
        data1 -> Nullable<Text>,
        data2 -> Nullable<Text>,
        grade -> Int8,
        submissioncomment -> Text,
        format -> Int2,
        teacher -> Int8,
        timemarked -> Int8,
        mailed -> Int2,
    }
}
*/
table! {
    mdl_assign_plugin_config (id) {
        id -> Int8,
        assignment -> Int8,
        plugin -> Varchar,
        subtype -> Varchar,
        name -> Varchar,
        value -> Nullable<Text>,
    }
}

table! {
    mdl_assign_submission (id) {
        id -> Int8,
        assignment -> Int8,
        userid -> Int8,
        timecreated -> Int8,
        timemodified -> Int8,
        status -> Nullable<Varchar>,
        groupid -> Int8,
    }
}

table! {
    mdl_assignsubmission_file (id) {
        id -> Int8,
        assignment -> Int8,
        submission -> Int8,
        numfiles -> Int8,
    }
}

table! {
    mdl_assignsubmission_onlinetext (id) {
        id -> Int8,
        assignment -> Int8,
        submission -> Int8,
        onlinetext -> Nullable<Text>,
        onlineformat -> Int2,
    }
}

table! {
    mdl_assign_user_mapping (id) {
        id -> Int8,
        assignment -> Int8,
        userid -> Int8,
    }
}

/* Ignore backup tables
table! {
    mdl_backup_controllers (id) {
        id -> Int8,
        backupid -> Varchar,
        operation -> Varchar,
        #[sql_name = "type"]
        type_ -> Varchar,
        itemid -> Int8,
        format -> Varchar,
        interactive -> Int2,
        purpose -> Int2,
        userid -> Int8,
        status -> Int2,
        execution -> Int2,
        executiontime -> Int8,
        checksum -> Varchar,
        timecreated -> Int8,
        timemodified -> Int8,
        controller -> Text,
    }
}

table! {
    mdl_backup_courses (id) {
        id -> Int8,
        courseid -> Int8,
        laststarttime -> Int8,
        lastendtime -> Int8,
        laststatus -> Varchar,
        nextstarttime -> Int8,
    }
}

table! {
    mdl_backup_files_template (id) {
        id -> Int8,
        backupid -> Varchar,
        contextid -> Int8,
        component -> Varchar,
        filearea -> Varchar,
        itemid -> Int8,
        info -> Nullable<Text>,
        newcontextid -> Nullable<Int8>,
        newitemid -> Nullable<Int8>,
    }
}

table! {
    mdl_backup_ids_template (id) {
        id -> Int8,
        backupid -> Varchar,
        itemname -> Varchar,
        itemid -> Int8,
        newitemid -> Int8,
        parentitemid -> Nullable<Int8>,
        info -> Nullable<Text>,
    }
}

table! {
    mdl_backup_logs (id) {
        id -> Int8,
        backupid -> Varchar,
        loglevel -> Int2,
        message -> Varchar,
        timecreated -> Int8,
    }
}
 */

table! {
    mdl_block (id) {
        id -> Int8,
        name -> Varchar,
        version -> Int8,
        cron -> Int8,
        lastcron -> Int8,
        visible -> Int2,
    }
}

table! {
    mdl_block_community (id) {
        id -> Int8,
        userid -> Int8,
        coursename -> Varchar,
        coursedescription -> Nullable<Text>,
        courseurl -> Varchar,
        imageurl -> Varchar,
    }
}

table! {
    mdl_block_instances (id) {
        id -> Int8,
        blockname -> Varchar,
        parentcontextid -> Int8,
        showinsubcontexts -> Int2,
        pagetypepattern -> Varchar,
        subpagepattern -> Nullable<Varchar>,
        defaultregion -> Varchar,
        defaultweight -> Int8,
        configdata -> Nullable<Text>,
    }
}

table! {
    mdl_block_positions (id) {
        id -> Int8,
        blockinstanceid -> Int8,
        contextid -> Int8,
        pagetype -> Varchar,
        subpage -> Varchar,
        visible -> Int2,
        region -> Varchar,
        weight -> Int8,
    }
}

table! {
    mdl_block_rss_client (id) {
        id -> Int8,
        userid -> Int8,
        title -> Text,
        preferredtitle -> Varchar,
        description -> Text,
        shared -> Int2,
        url -> Varchar,
    }
}

/* Empty
table! {
    mdl_blog_association (id) {
        id -> Int8,
        contextid -> Int8,
        blogid -> Int8,
    }
}
 */
/* Empty
table! {
    mdl_blog_external (id) {
        id -> Int8,
        userid -> Int8,
        name -> Varchar,
        description -> Nullable<Text>,
        url -> Text,
        filtertags -> Nullable<Varchar>,
        failedlastsync -> Int2,
        timemodified -> Nullable<Int8>,
        timefetched -> Int8,
    }
}
 */

/* Empty
table! {
    mdl_book (id) {
        id -> Int8,
        course -> Int8,
        name -> Varchar,
        intro -> Nullable<Text>,
        introformat -> Int2,
        numbering -> Int2,
        customtitles -> Int2,
        revision -> Int8,
        timecreated -> Int8,
        timemodified -> Int8,
    }
}

table! {
    mdl_book_chapters (id) {
        id -> Int8,
        bookid -> Int8,
        pagenum -> Int8,
        subchapter -> Int8,
        title -> Varchar,
        content -> Text,
        contentformat -> Int2,
        hidden -> Int2,
        timecreated -> Int8,
        timemodified -> Int8,
        importsrc -> Varchar,
    }
}
 */

table! {
    mdl_cache_filters (id) {
        id -> Int8,
        filter -> Varchar,
        version -> Int8,
        md5key -> Varchar,
        rawtext -> Text,
        timemodified -> Int8,
    }
}

table! {
    mdl_cache_flags (id) {
        id -> Int8,
        flagtype -> Varchar,
        name -> Varchar,
        timemodified -> Int8,
        value -> Text,
        expiry -> Int8,
    }
}

table! {
    mdl_cache_text (id) {
        id -> Int8,
        md5key -> Varchar,
        formattedtext -> Text,
        timemodified -> Int8,
    }
}

table! {
    mdl_capabilities (id) {
        id -> Int8,
        name -> Varchar,
        captype -> Varchar,
        contextlevel -> Int8,
        component -> Varchar,
        riskbitmask -> Int8,
    }
}

table! {
    mdl_chat (id) {
        id -> Int8,
        course -> Int8,
        name -> Varchar,
        intro -> Text,
        introformat -> Int2,
        keepdays -> Int8,
        studentlogs -> Int2,
        chattime -> Int8,
        schedule -> Int2,
        timemodified -> Int8,
    }
}

table! {
    mdl_chat_messages (id) {
        id -> Int8,
        chatid -> Int8,
        userid -> Int8,
        groupid -> Int8,
        system -> Int2,
        message -> Text,
        timestamp -> Int8,
    }
}

table! {
    mdl_chat_messages_current (id) {
        id -> Int8,
        chatid -> Int8,
        userid -> Int8,
        groupid -> Int8,
        system -> Int2,
        message -> Text,
        timestamp -> Int8,
    }
}

table! {
    mdl_chat_users (id) {
        id -> Int8,
        chatid -> Int8,
        userid -> Int8,
        groupid -> Int8,
        version -> Varchar,
        ip -> Varchar,
        firstping -> Int8,
        lastping -> Int8,
        lastmessageping -> Int8,
        sid -> Varchar,
        course -> Int8,
        lang -> Varchar,
    }
}

/* Empty
table! {
    mdl_choice (id) {
        id -> Int8,
        course -> Int8,
        name -> Varchar,
        intro -> Text,
        introformat -> Int2,
        publish -> Int2,
        showresults -> Int2,
        display -> Int2,
        allowupdate -> Int2,
        showunanswered -> Int2,
        limitanswers -> Int2,
        timeopen -> Int8,
        timeclose -> Int8,
        timemodified -> Int8,
        completionsubmit -> Int2,
    }
}
table! {
    mdl_choice_answers (id) {
        id -> Int8,
        choiceid -> Int8,
        userid -> Int8,
        optionid -> Int8,
        timemodified -> Int8,
    }
}

table! {
    mdl_choice_options (id) {
        id -> Int8,
        choiceid -> Int8,
        text -> Nullable<Text>,
        maxanswers -> Nullable<Int8>,
        timemodified -> Int8,
    }
}
*/

table! {
    mdl_cohort (id) {
        id -> Int8,
        contextid -> Int8,
        name -> Varchar,
        idnumber -> Nullable<Varchar>,
        description -> Nullable<Text>,
        descriptionformat -> Int2,
        component -> Varchar,
        timecreated -> Int8,
        timemodified -> Int8,
    }
}

table! {
    mdl_cohort_members (id) {
        id -> Int8,
        cohortid -> Int8,
        userid -> Int8,
        timeadded -> Int8,
    }
}

// Verkar vara enskilda studenters kommentarer om vad de lämnat in.
table! {
    mdl_comments (id) {
        id -> Int8,
        contextid -> Int8,
        commentarea -> Varchar,
        itemid -> Int8,
        content -> Text,
        format -> Int2,
        userid -> Int8,
        timecreated -> Int8,
    }
}

table! {
    mdl_config (id) {
        id -> Int8,
        name -> Varchar,
        value -> Text,
    }
}

table! {
    mdl_config_log (id) {
        id -> Int8,
        userid -> Int8,
        timemodified -> Int8,
        plugin -> Nullable<Varchar>,
        name -> Varchar,
        value -> Nullable<Text>,
        oldvalue -> Nullable<Text>,
    }
}

table! {
    mdl_config_plugins (id) {
        id -> Int8,
        plugin -> Varchar,
        name -> Varchar,
        value -> Text,
    }
}

table! {
    mdl_context (id) {
        id -> Int8,
        contextlevel -> Int8,
        instanceid -> Int8,
        path -> Nullable<Varchar>,
        depth -> Int2,
    }
}

table! {
    mdl_context_temp (id) {
        id -> Int8,
        path -> Varchar,
        depth -> Int2,
    }
}

table! {
    mdl_course (id) {
        id -> Int8,
        category -> Int8,
        sortorder -> Int8,
        fullname -> Varchar,
        shortname -> Varchar,
        idnumber -> Varchar,
        summary -> Nullable<Text>,
        summaryformat -> Int2,
        format -> Varchar,
        showgrades -> Int2,
        sectioncache -> Nullable<Text>,
        modinfo -> Nullable<Text>,
        newsitems -> Int4,
        startdate -> Int8,
        marker -> Int8,
        maxbytes -> Int8,
        legacyfiles -> Int2,
        showreports -> Int2,
        visible -> Int2,
        visibleold -> Int2,
        groupmode -> Int2,
        groupmodeforce -> Int2,
        defaultgroupingid -> Int8,
        lang -> Varchar,
        theme -> Varchar,
        timecreated -> Int8,
        timemodified -> Int8,
        requested -> Int2,
        enablecompletion -> Int2,
        completionstartonenrol -> Int2,
        completionnotify -> Int2,
    }
}

table! {
    mdl_course_categories (id) {
        id -> Int8,
        name -> Varchar,
        idnumber -> Nullable<Varchar>,
        description -> Nullable<Text>,
        descriptionformat -> Int2,
        parent -> Int8,
        sortorder -> Int8,
        coursecount -> Int8,
        visible -> Int2,
        visibleold -> Int2,
        timemodified -> Int8,
        depth -> Int8,
        path -> Varchar,
        theme -> Nullable<Varchar>,
    }
}

/* Empty
table! {
    mdl_course_completion_aggr_methd (id) {
        id -> Int8,
        course -> Int8,
        criteriatype -> Nullable<Int8>,
        method -> Int2,
        value -> Nullable<Numeric>,
    }
}
*/
/* Empty
table! {
    mdl_course_completion_crit_compl (id) {
        id -> Int8,
        userid -> Int8,
        course -> Int8,
        criteriaid -> Int8,
        gradefinal -> Nullable<Numeric>,
        unenroled -> Nullable<Int8>,
        timecompleted -> Nullable<Int8>,
    }
}
*/
/* Empty
table! {
    mdl_course_completion_criteria (id) {
        id -> Int8,
        course -> Int8,
        criteriatype -> Int8,
        module -> Nullable<Varchar>,
        moduleinstance -> Nullable<Int8>,
        courseinstance -> Nullable<Int8>,
        enrolperiod -> Nullable<Int8>,
        timeend -> Nullable<Int8>,
        gradepass -> Nullable<Numeric>,
        role -> Nullable<Int8>,
    }
}
 */
/* Empty
table! {
    mdl_course_completions (id) {
        id -> Int8,
        userid -> Int8,
        course -> Int8,
        timeenrolled -> Int8,
        timestarted -> Int8,
        timecompleted -> Nullable<Int8>,
        reaggregate -> Int8,
    }
}
*/
table! {
    mdl_course_format_options (id) {
        id -> Int8,
        courseid -> Int8,
        format -> Varchar,
        sectionid -> Int8,
        name -> Varchar,
        value -> Nullable<Text>,
    }
}

table! {
    mdl_course_modules (id) {
        id -> Int8,
        course -> Int8,
        module -> Int8,
        instance -> Int8,
        section -> Int8,
        idnumber -> Nullable<Varchar>,
        added -> Int8,
        score -> Int2,
        indent -> Int4,
        visible -> Int2,
        visibleold -> Int2,
        groupmode -> Int2,
        groupingid -> Int8,
        groupmembersonly -> Int2,
        completion -> Int2,
        completiongradeitemnumber -> Nullable<Int8>,
        completionview -> Int2,
        completionexpected -> Int8,
        availablefrom -> Int8,
        availableuntil -> Int8,
        showavailability -> Int2,
        showdescription -> Int2,
    }
}

table! {
    mdl_course_modules_availability (id) {
        id -> Int8,
        coursemoduleid -> Int8,
        sourcecmid -> Nullable<Int8>,
        requiredcompletion -> Nullable<Int2>,
        gradeitemid -> Nullable<Int8>,
        grademin -> Nullable<Numeric>,
        grademax -> Nullable<Numeric>,
    }
}

table! {
    mdl_course_modules_avail_fields (id) {
        id -> Int8,
        coursemoduleid -> Int8,
        userfield -> Nullable<Varchar>,
        customfieldid -> Nullable<Int8>,
        operator -> Varchar,
        value -> Varchar,
    }
}

table! {
    mdl_course_modules_completion (id) {
        id -> Int8,
        coursemoduleid -> Int8,
        userid -> Int8,
        completionstate -> Int2,
        viewed -> Nullable<Int2>,
        timemodified -> Int8,
    }
}

table! {
    mdl_course_published (id) {
        id -> Int8,
        huburl -> Nullable<Varchar>,
        courseid -> Int8,
        timepublished -> Int8,
        enrollable -> Int2,
        hubcourseid -> Int8,
        status -> Nullable<Int2>,
        timechecked -> Nullable<Int8>,
    }
}

table! {
    mdl_course_request (id) {
        id -> Int8,
        fullname -> Varchar,
        shortname -> Varchar,
        summary -> Text,
        summaryformat -> Int2,
        category -> Int8,
        reason -> Text,
        requester -> Int8,
        password -> Varchar,
    }
}

table! {
    mdl_course_sections (id) {
        id -> Int8,
        course -> Int8,
        section -> Int8,
        name -> Nullable<Varchar>,
        summary -> Nullable<Text>,
        summaryformat -> Int2,
        sequence -> Nullable<Text>,
        visible -> Int2,
        availablefrom -> Int8,
        availableuntil -> Int8,
        showavailability -> Int2,
        groupingid -> Int8,
    }
}

table! {
    mdl_course_sections_availability (id) {
        id -> Int8,
        coursesectionid -> Int8,
        sourcecmid -> Nullable<Int8>,
        requiredcompletion -> Nullable<Int2>,
        gradeitemid -> Nullable<Int8>,
        grademin -> Nullable<Numeric>,
        grademax -> Nullable<Numeric>,
    }
}

table! {
    mdl_course_sections_avail_fields (id) {
        id -> Int8,
        coursesectionid -> Int8,
        userfield -> Nullable<Varchar>,
        customfieldid -> Nullable<Int8>,
        operator -> Varchar,
        value -> Varchar,
    }
}

/* Empty
table! {
    mdl_data (id) {
        id -> Int8,
        course -> Int8,
        name -> Varchar,
        intro -> Text,
        introformat -> Int2,
        comments -> Int2,
        timeavailablefrom -> Int8,
        timeavailableto -> Int8,
        timeviewfrom -> Int8,
        timeviewto -> Int8,
        requiredentries -> Int4,
        requiredentriestoview -> Int4,
        maxentries -> Int4,
        rssarticles -> Int2,
        singletemplate -> Nullable<Text>,
        listtemplate -> Nullable<Text>,
        listtemplateheader -> Nullable<Text>,
        listtemplatefooter -> Nullable<Text>,
        addtemplate -> Nullable<Text>,
        rsstemplate -> Nullable<Text>,
        rsstitletemplate -> Nullable<Text>,
        csstemplate -> Nullable<Text>,
        jstemplate -> Nullable<Text>,
        asearchtemplate -> Nullable<Text>,
        approval -> Int2,
        scale -> Int8,
        assessed -> Int8,
        assesstimestart -> Int8,
        assesstimefinish -> Int8,
        defaultsort -> Int8,
        defaultsortdir -> Int2,
        editany -> Int2,
        notification -> Int8,
    }
}
 */
/* Empty
table! {
    mdl_data_content (id) {
        id -> Int8,
        fieldid -> Int8,
        recordid -> Int8,
        content -> Nullable<Text>,
        content1 -> Nullable<Text>,
        content2 -> Nullable<Text>,
        content3 -> Nullable<Text>,
        content4 -> Nullable<Text>,
    }
}
 */
/* Empty
table! {
    mdl_data_fields (id) {
        id -> Int8,
        dataid -> Int8,
        #[sql_name = "type"]
        type_ -> Varchar,
        name -> Varchar,
        description -> Text,
        param1 -> Nullable<Text>,
        param2 -> Nullable<Text>,
        param3 -> Nullable<Text>,
        param4 -> Nullable<Text>,
        param5 -> Nullable<Text>,
        param6 -> Nullable<Text>,
        param7 -> Nullable<Text>,
        param8 -> Nullable<Text>,
        param9 -> Nullable<Text>,
        param10 -> Nullable<Text>,
    }
}
*/
/* Empty
table! {
    mdl_data_records (id) {
        id -> Int8,
        userid -> Int8,
        groupid -> Int8,
        dataid -> Int8,
        timecreated -> Int8,
        timemodified -> Int8,
        approved -> Int2,
    }
}
*/
table! {
    mdl_enrol (id) {
        id -> Int8,
        enrol -> Varchar,
        status -> Int8,
        courseid -> Int8,
        sortorder -> Int8,
        name -> Nullable<Varchar>,
        enrolperiod -> Nullable<Int8>,
        enrolstartdate -> Nullable<Int8>,
        enrolenddate -> Nullable<Int8>,
        expirynotify -> Nullable<Int2>,
        expirythreshold -> Nullable<Int8>,
        notifyall -> Nullable<Int2>,
        password -> Nullable<Varchar>,
        cost -> Nullable<Varchar>,
        currency -> Nullable<Varchar>,
        roleid -> Nullable<Int8>,
        customint1 -> Nullable<Int8>,
        customint2 -> Nullable<Int8>,
        customint3 -> Nullable<Int8>,
        customint4 -> Nullable<Int8>,
        customint5 -> Nullable<Int8>,
        customint6 -> Nullable<Int8>,
        customint7 -> Nullable<Int8>,
        customint8 -> Nullable<Int8>,
        customchar1 -> Nullable<Varchar>,
        customchar2 -> Nullable<Varchar>,
        customchar3 -> Nullable<Varchar>,
        customdec1 -> Nullable<Numeric>,
        customdec2 -> Nullable<Numeric>,
        customtext1 -> Nullable<Text>,
        customtext2 -> Nullable<Text>,
        customtext3 -> Nullable<Text>,
        customtext4 -> Nullable<Text>,
        timecreated -> Int8,
        timemodified -> Int8,
    }
}

table! {
    mdl_enrol_authorize (id) {
        id -> Int8,
        paymentmethod -> Varchar,
        refundinfo -> Int2,
        ccname -> Varchar,
        courseid -> Int8,
        instanceid -> Int8,
        userid -> Int8,
        transid -> Int8,
        status -> Int8,
        timecreated -> Int8,
        settletime -> Int8,
        amount -> Varchar,
        currency -> Varchar,
    }
}

table! {
    mdl_enrol_authorize_refunds (id) {
        id -> Int8,
        orderid -> Int8,
        status -> Int2,
        amount -> Varchar,
        transid -> Nullable<Int8>,
        settletime -> Int8,
    }
}

table! {
    mdl_enrol_flatfile (id) {
        id -> Int8,
        action -> Varchar,
        roleid -> Int8,
        userid -> Int8,
        courseid -> Int8,
        timestart -> Int8,
        timeend -> Int8,
        timemodified -> Int8,
    }
}

table! {
    mdl_enrol_paypal (id) {
        id -> Int8,
        business -> Varchar,
        receiver_email -> Varchar,
        receiver_id -> Varchar,
        item_name -> Varchar,
        courseid -> Int8,
        userid -> Int8,
        instanceid -> Int8,
        memo -> Varchar,
        tax -> Varchar,
        option_name1 -> Varchar,
        option_selection1_x -> Varchar,
        option_name2 -> Varchar,
        option_selection2_x -> Varchar,
        payment_status -> Varchar,
        pending_reason -> Varchar,
        reason_code -> Varchar,
        txn_id -> Varchar,
        parent_txn_id -> Varchar,
        payment_type -> Varchar,
        timeupdated -> Int8,
    }
}

table! {
    mdl_event (id) {
        id -> Int8,
        name -> Text,
        description -> Text,
        format -> Int2,
        courseid -> Int8,
        groupid -> Int8,
        userid -> Int8,
        repeatid -> Int8,
        modulename -> Varchar,
        instance -> Int8,
        eventtype -> Varchar,
        timestart -> Int8,
        timeduration -> Int8,
        visible -> Int2,
        uuid -> Varchar,
        sequence -> Int8,
        timemodified -> Int8,
        subscriptionid -> Nullable<Int8>,
    }
}

table! {
    mdl_events_handlers (id) {
        id -> Int8,
        eventname -> Varchar,
        component -> Varchar,
        handlerfile -> Varchar,
        handlerfunction -> Nullable<Text>,
        schedule -> Nullable<Varchar>,
        status -> Int8,
        internal -> Int2,
    }
}

table! {
    mdl_events_queue (id) {
        id -> Int8,
        eventdata -> Text,
        stackdump -> Nullable<Text>,
        userid -> Nullable<Int8>,
        timecreated -> Int8,
    }
}

table! {
    mdl_events_queue_handlers (id) {
        id -> Int8,
        queuedeventid -> Int8,
        handlerid -> Int8,
        status -> Nullable<Int8>,
        errormessage -> Nullable<Text>,
        timemodified -> Int8,
    }
}

table! {
    mdl_event_subscriptions (id) {
        id -> Int8,
        url -> Varchar,
        courseid -> Int8,
        groupid -> Int8,
        userid -> Int8,
        eventtype -> Varchar,
        pollinterval -> Int8,
        lastupdated -> Nullable<Int8>,
        name -> Varchar,
    }
}

table! {
    mdl_external_functions (id) {
        id -> Int8,
        name -> Varchar,
        classname -> Varchar,
        methodname -> Varchar,
        classpath -> Nullable<Varchar>,
        component -> Varchar,
        capabilities -> Nullable<Varchar>,
    }
}

table! {
    mdl_external_services (id) {
        id -> Int8,
        name -> Varchar,
        enabled -> Int2,
        requiredcapability -> Nullable<Varchar>,
        restrictedusers -> Int2,
        component -> Nullable<Varchar>,
        timecreated -> Int8,
        timemodified -> Nullable<Int8>,
        shortname -> Nullable<Varchar>,
        downloadfiles -> Int2,
    }
}

table! {
    mdl_external_services_functions (id) {
        id -> Int8,
        externalserviceid -> Int8,
        functionname -> Varchar,
    }
}

table! {
    mdl_external_services_users (id) {
        id -> Int8,
        externalserviceid -> Int8,
        userid -> Int8,
        iprestriction -> Nullable<Varchar>,
        validuntil -> Nullable<Int8>,
        timecreated -> Nullable<Int8>,
    }
}

table! {
    mdl_external_tokens (id) {
        id -> Int8,
        token -> Varchar,
        tokentype -> Int2,
        userid -> Int8,
        externalserviceid -> Int8,
        sid -> Nullable<Varchar>,
        contextid -> Int8,
        creatorid -> Int8,
        iprestriction -> Nullable<Varchar>,
        validuntil -> Nullable<Int8>,
        timecreated -> Int8,
        lastaccess -> Nullable<Int8>,
    }
}

table! {
    mdl_feedback (id) {
        id -> Int8,
        course -> Int8,
        name -> Varchar,
        intro -> Text,
        introformat -> Int2,
        anonymous -> Int2,
        email_notification -> Int2,
        multiple_submit -> Int2,
        autonumbering -> Int2,
        site_after_submit -> Varchar,
        page_after_submit -> Text,
        page_after_submitformat -> Int2,
        publish_stats -> Int2,
        timeopen -> Int8,
        timeclose -> Int8,
        timemodified -> Int8,
        completionsubmit -> Int2,
    }
}

table! {
    mdl_feedback_completed (id) {
        id -> Int8,
        feedback -> Int8,
        userid -> Int8,
        timemodified -> Int8,
        random_response -> Int8,
        anonymous_response -> Int2,
    }
}

table! {
    mdl_feedback_completedtmp (id) {
        id -> Int8,
        feedback -> Int8,
        userid -> Int8,
        guestid -> Varchar,
        timemodified -> Int8,
        random_response -> Int8,
        anonymous_response -> Int2,
    }
}

table! {
    mdl_feedback_item (id) {
        id -> Int8,
        feedback -> Int8,
        template -> Int8,
        name -> Varchar,
        label -> Varchar,
        presentation -> Text,
        typ -> Varchar,
        hasvalue -> Int2,
        position -> Int2,
        required -> Int2,
        dependitem -> Int8,
        dependvalue -> Varchar,
        options -> Varchar,
    }
}

table! {
    mdl_feedback_sitecourse_map (id) {
        id -> Int8,
        feedbackid -> Int8,
        courseid -> Int8,
    }
}

table! {
    mdl_feedback_template (id) {
        id -> Int8,
        course -> Int8,
        ispublic -> Int2,
        name -> Varchar,
    }
}

table! {
    mdl_feedback_tracking (id) {
        id -> Int8,
        userid -> Int8,
        feedback -> Int8,
        completed -> Int8,
        tmp_completed -> Int8,
    }
}

table! {
    mdl_feedback_value (id) {
        id -> Int8,
        course_id -> Int8,
        item -> Int8,
        completed -> Int8,
        tmp_completed -> Int8,
        value -> Text,
    }
}

table! {
    mdl_feedback_valuetmp (id) {
        id -> Int8,
        course_id -> Int8,
        item -> Int8,
        completed -> Int8,
        tmp_completed -> Int8,
        value -> Text,
    }
}

table! {
    mdl_files (id) {
        id -> Int8,
        contenthash -> Varchar,
        pathnamehash -> Varchar,
        contextid -> Int8,
        component -> Varchar,
        filearea -> Varchar,
        itemid -> Int8,
        filepath -> Varchar,
        filename -> Varchar,
        userid -> Nullable<Int8>,
        filesize -> Int8,
        mimetype -> Nullable<Varchar>,
        status -> Int8,
        source -> Nullable<Text>,
        author -> Nullable<Varchar>,
        license -> Nullable<Varchar>,
        timecreated -> Int8,
        timemodified -> Int8,
        sortorder -> Int8,
        referencefileid -> Nullable<Int8>,
        referencelastsync -> Nullable<Int8>,
        referencelifetime -> Nullable<Int8>,
    }
}

table! {
    mdl_files_reference (id) {
        id -> Int8,
        repositoryid -> Int8,
        lastsync -> Nullable<Int8>,
        lifetime -> Nullable<Int8>,
        reference -> Nullable<Text>,
        referencehash -> Varchar,
    }
}

table! {
    mdl_filter_active (id) {
        id -> Int8,
        filter -> Varchar,
        contextid -> Int8,
        active -> Int2,
        sortorder -> Int8,
    }
}

table! {
    mdl_filter_config (id) {
        id -> Int8,
        filter -> Varchar,
        contextid -> Int8,
        name -> Varchar,
        value -> Nullable<Text>,
    }
}

table! {
    mdl_folder (id) {
        id -> Int8,
        course -> Int8,
        name -> Varchar,
        intro -> Nullable<Text>,
        introformat -> Int2,
        revision -> Int8,
        timemodified -> Int8,
    }
}

table! {
    mdl_forum (id) {
        id -> Int8,
        course -> Int8,
        #[sql_name = "type"]
        type_ -> Varchar,
        name -> Varchar,
        intro -> Text,
        introformat -> Int2,
        assessed -> Int8,
        assesstimestart -> Int8,
        assesstimefinish -> Int8,
        scale -> Int8,
        maxbytes -> Int8,
        maxattachments -> Int8,
        forcesubscribe -> Int2,
        trackingtype -> Int2,
        rsstype -> Int2,
        rssarticles -> Int2,
        timemodified -> Int8,
        warnafter -> Int8,
        blockafter -> Int8,
        blockperiod -> Int8,
        completiondiscussions -> Int4,
        completionreplies -> Int4,
        completionposts -> Int4,
    }
}

table! {
    mdl_forum_discussions (id) {
        id -> Int8,
        course -> Int8,
        forum -> Int8,
        name -> Varchar,
        firstpost -> Int8,
        userid -> Int8,
        groupid -> Int8,
        assessed -> Int2,
        timemodified -> Int8,
        usermodified -> Int8,
        timestart -> Int8,
        timeend -> Int8,
    }
}

table! {
    mdl_forum_posts (id) {
        id -> Int8,
        discussion -> Int8,
        parent -> Int8,
        userid -> Int8,
        created -> Int8,
        modified -> Int8,
        mailed -> Int2,
        subject -> Varchar,
        message -> Text,
        messageformat -> Int2,
        messagetrust -> Int2,
        attachment -> Varchar,
        totalscore -> Int2,
        mailnow -> Int8,
    }
}

table! {
    mdl_forum_queue (id) {
        id -> Int8,
        userid -> Int8,
        discussionid -> Int8,
        postid -> Int8,
        timemodified -> Int8,
    }
}

table! {
    mdl_forum_read (id) {
        id -> Int8,
        userid -> Int8,
        forumid -> Int8,
        discussionid -> Int8,
        postid -> Int8,
        firstread -> Int8,
        lastread -> Int8,
    }
}

table! {
    mdl_forum_subscriptions (id) {
        id -> Int8,
        userid -> Int8,
        forum -> Int8,
    }
}

table! {
    mdl_forum_track_prefs (id) {
        id -> Int8,
        userid -> Int8,
        forumid -> Int8,
    }
}

table! {
    mdl_glossary (id) {
        id -> Int8,
        course -> Int8,
        name -> Varchar,
        intro -> Text,
        introformat -> Int2,
        allowduplicatedentries -> Int2,
        displayformat -> Varchar,
        mainglossary -> Int2,
        showspecial -> Int2,
        showalphabet -> Int2,
        showall -> Int2,
        allowcomments -> Int2,
        allowprintview -> Int2,
        usedynalink -> Int2,
        defaultapproval -> Int2,
        approvaldisplayformat -> Varchar,
        globalglossary -> Int2,
        entbypage -> Int2,
        editalways -> Int2,
        rsstype -> Int2,
        rssarticles -> Int2,
        assessed -> Int8,
        assesstimestart -> Int8,
        assesstimefinish -> Int8,
        scale -> Int8,
        timecreated -> Int8,
        timemodified -> Int8,
        completionentries -> Int4,
    }
}

table! {
    mdl_glossary_alias (id) {
        id -> Int8,
        entryid -> Int8,
        alias -> Varchar,
    }
}

table! {
    mdl_glossary_categories (id) {
        id -> Int8,
        glossaryid -> Int8,
        name -> Varchar,
        usedynalink -> Int2,
    }
}

table! {
    mdl_glossary_entries (id) {
        id -> Int8,
        glossaryid -> Int8,
        userid -> Int8,
        concept -> Varchar,
        definition -> Text,
        definitionformat -> Int2,
        definitiontrust -> Int2,
        attachment -> Varchar,
        timecreated -> Int8,
        timemodified -> Int8,
        teacherentry -> Int2,
        sourceglossaryid -> Int8,
        usedynalink -> Int2,
        casesensitive -> Int2,
        fullmatch -> Int2,
        approved -> Int2,
    }
}

table! {
    mdl_glossary_entries_categories (id) {
        id -> Int8,
        categoryid -> Int8,
        entryid -> Int8,
    }
}

table! {
    mdl_glossary_formats (id) {
        id -> Int8,
        name -> Varchar,
        popupformatname -> Varchar,
        visible -> Int2,
        showgroup -> Int2,
        defaultmode -> Varchar,
        defaulthook -> Varchar,
        sortkey -> Varchar,
        sortorder -> Varchar,
    }
}

table! {
    mdl_grade_categories (id) {
        id -> Int8,
        courseid -> Int8,
        parent -> Nullable<Int8>,
        depth -> Int8,
        path -> Nullable<Varchar>,
        fullname -> Varchar,
        aggregation -> Int8,
        keephigh -> Int8,
        droplow -> Int8,
        aggregateonlygraded -> Int2,
        aggregateoutcomes -> Int2,
        aggregatesubcats -> Int2,
        timecreated -> Int8,
        timemodified -> Int8,
        hidden -> Int2,
    }
}

table! {
    mdl_grade_categories_history (id) {
        id -> Int8,
        action -> Int8,
        oldid -> Int8,
        source -> Nullable<Varchar>,
        timemodified -> Nullable<Int8>,
        loggeduser -> Nullable<Int8>,
        courseid -> Int8,
        parent -> Nullable<Int8>,
        depth -> Int8,
        path -> Nullable<Varchar>,
        fullname -> Varchar,
        aggregation -> Int8,
        keephigh -> Int8,
        droplow -> Int8,
        aggregateonlygraded -> Int2,
        aggregateoutcomes -> Int2,
        aggregatesubcats -> Int2,
        hidden -> Int2,
    }
}

table! {
    mdl_grade_grades (id) {
        id -> Int8,
        itemid -> Int8,
        userid -> Int8,
        rawgrade -> Nullable<Numeric>,
        rawgrademax -> Numeric,
        rawgrademin -> Numeric,
        rawscaleid -> Nullable<Int8>,
        usermodified -> Nullable<Int8>,
        finalgrade -> Nullable<Numeric>,
        hidden -> Int8,
        locked -> Int8,
        locktime -> Int8,
        exported -> Int8,
        overridden -> Int8,
        excluded -> Int8,
        feedback -> Nullable<Text>,
        feedbackformat -> Int8,
        information -> Nullable<Text>,
        informationformat -> Int8,
        timecreated -> Nullable<Int8>,
        timemodified -> Nullable<Int8>,
    }
}

table! {
    mdl_grade_grades_history (id) {
        id -> Int8,
        action -> Int8,
        oldid -> Int8,
        source -> Nullable<Varchar>,
        timemodified -> Nullable<Int8>,
        loggeduser -> Nullable<Int8>,
        itemid -> Int8,
        userid -> Int8,
        rawgrade -> Nullable<Numeric>,
        rawgrademax -> Numeric,
        rawgrademin -> Numeric,
        rawscaleid -> Nullable<Int8>,
        usermodified -> Nullable<Int8>,
        finalgrade -> Nullable<Numeric>,
        hidden -> Int8,
        locked -> Int8,
        locktime -> Int8,
        exported -> Int8,
        overridden -> Int8,
        excluded -> Int8,
        feedback -> Nullable<Text>,
        feedbackformat -> Int8,
        information -> Nullable<Text>,
        informationformat -> Int8,
    }
}

table! {
    mdl_grade_import_newitem (id) {
        id -> Int8,
        itemname -> Varchar,
        importcode -> Int8,
        importer -> Int8,
    }
}

table! {
    mdl_grade_import_values (id) {
        id -> Int8,
        itemid -> Nullable<Int8>,
        newgradeitem -> Nullable<Int8>,
        userid -> Int8,
        finalgrade -> Nullable<Numeric>,
        feedback -> Nullable<Text>,
        importcode -> Int8,
        importer -> Nullable<Int8>,
    }
}

table! {
    mdl_grade_items (id) {
        id -> Int8,
        courseid -> Nullable<Int8>,
        categoryid -> Nullable<Int8>,
        itemname -> Nullable<Varchar>,
        itemtype -> Varchar,
        itemmodule -> Nullable<Varchar>,
        iteminstance -> Nullable<Int8>,
        itemnumber -> Nullable<Int8>,
        iteminfo -> Nullable<Text>,
        idnumber -> Nullable<Varchar>,
        calculation -> Nullable<Text>,
        gradetype -> Int2,
        grademax -> Numeric,
        grademin -> Numeric,
        scaleid -> Nullable<Int8>,
        outcomeid -> Nullable<Int8>,
        gradepass -> Numeric,
        multfactor -> Numeric,
        plusfactor -> Numeric,
        aggregationcoef -> Numeric,
        sortorder -> Int8,
        display -> Int8,
        decimals -> Nullable<Int2>,
        hidden -> Int8,
        locked -> Int8,
        locktime -> Int8,
        needsupdate -> Int8,
        timecreated -> Nullable<Int8>,
        timemodified -> Nullable<Int8>,
    }
}

table! {
    mdl_grade_items_history (id) {
        id -> Int8,
        action -> Int8,
        oldid -> Int8,
        source -> Nullable<Varchar>,
        timemodified -> Nullable<Int8>,
        loggeduser -> Nullable<Int8>,
        courseid -> Nullable<Int8>,
        categoryid -> Nullable<Int8>,
        itemname -> Nullable<Varchar>,
        itemtype -> Varchar,
        itemmodule -> Nullable<Varchar>,
        iteminstance -> Nullable<Int8>,
        itemnumber -> Nullable<Int8>,
        iteminfo -> Nullable<Text>,
        idnumber -> Nullable<Varchar>,
        calculation -> Nullable<Text>,
        gradetype -> Int2,
        grademax -> Numeric,
        grademin -> Numeric,
        scaleid -> Nullable<Int8>,
        outcomeid -> Nullable<Int8>,
        gradepass -> Numeric,
        multfactor -> Numeric,
        plusfactor -> Numeric,
        aggregationcoef -> Numeric,
        sortorder -> Int8,
        hidden -> Int8,
        locked -> Int8,
        locktime -> Int8,
        needsupdate -> Int8,
        display -> Int8,
        decimals -> Nullable<Int2>,
    }
}

table! {
    mdl_grade_letters (id) {
        id -> Int8,
        contextid -> Int8,
        lowerboundary -> Numeric,
        letter -> Varchar,
    }
}

table! {
    mdl_grade_outcomes (id) {
        id -> Int8,
        courseid -> Nullable<Int8>,
        shortname -> Varchar,
        fullname -> Text,
        scaleid -> Nullable<Int8>,
        description -> Nullable<Text>,
        descriptionformat -> Int2,
        timecreated -> Nullable<Int8>,
        timemodified -> Nullable<Int8>,
        usermodified -> Nullable<Int8>,
    }
}

table! {
    mdl_grade_outcomes_courses (id) {
        id -> Int8,
        courseid -> Int8,
        outcomeid -> Int8,
    }
}

table! {
    mdl_grade_outcomes_history (id) {
        id -> Int8,
        action -> Int8,
        oldid -> Int8,
        source -> Nullable<Varchar>,
        timemodified -> Nullable<Int8>,
        loggeduser -> Nullable<Int8>,
        courseid -> Nullable<Int8>,
        shortname -> Varchar,
        fullname -> Text,
        scaleid -> Nullable<Int8>,
        description -> Nullable<Text>,
        descriptionformat -> Int2,
    }
}

table! {
    mdl_grade_settings (id) {
        id -> Int8,
        courseid -> Int8,
        name -> Varchar,
        value -> Nullable<Text>,
    }
}

table! {
    mdl_grading_areas (id) {
        id -> Int8,
        contextid -> Int8,
        component -> Varchar,
        areaname -> Varchar,
        activemethod -> Nullable<Varchar>,
    }
}

table! {
    mdl_grading_definitions (id) {
        id -> Int8,
        areaid -> Int8,
        method -> Varchar,
        name -> Varchar,
        description -> Nullable<Text>,
        descriptionformat -> Nullable<Int2>,
        status -> Int8,
        copiedfromid -> Nullable<Int8>,
        timecreated -> Int8,
        usercreated -> Int8,
        timemodified -> Int8,
        usermodified -> Int8,
        timecopied -> Nullable<Int8>,
        options -> Nullable<Text>,
    }
}

table! {
    mdl_gradingform_guide_comments (id) {
        id -> Int8,
        definitionid -> Int8,
        sortorder -> Int8,
        description -> Nullable<Text>,
        descriptionformat -> Nullable<Int2>,
    }
}

table! {
    mdl_gradingform_guide_criteria (id) {
        id -> Int8,
        definitionid -> Int8,
        sortorder -> Int8,
        shortname -> Varchar,
        description -> Nullable<Text>,
        descriptionformat -> Nullable<Int2>,
        descriptionmarkers -> Nullable<Text>,
        descriptionmarkersformat -> Nullable<Int2>,
        maxscore -> Numeric,
    }
}

table! {
    mdl_gradingform_guide_fillings (id) {
        id -> Int8,
        instanceid -> Int8,
        criterionid -> Int8,
        remark -> Nullable<Text>,
        remarkformat -> Nullable<Int2>,
        score -> Numeric,
    }
}

table! {
    mdl_gradingform_rubric_criteria (id) {
        id -> Int8,
        definitionid -> Int8,
        sortorder -> Int8,
        description -> Nullable<Text>,
        descriptionformat -> Nullable<Int2>,
    }
}

table! {
    mdl_gradingform_rubric_fillings (id) {
        id -> Int8,
        instanceid -> Int8,
        criterionid -> Int8,
        levelid -> Nullable<Int8>,
        remark -> Nullable<Text>,
        remarkformat -> Nullable<Int2>,
    }
}

table! {
    mdl_gradingform_rubric_levels (id) {
        id -> Int8,
        criterionid -> Int8,
        score -> Numeric,
        definition -> Nullable<Text>,
        definitionformat -> Nullable<Int8>,
    }
}

table! {
    mdl_grading_instances (id) {
        id -> Int8,
        definitionid -> Int8,
        raterid -> Int8,
        itemid -> Nullable<Int8>,
        rawgrade -> Nullable<Numeric>,
        status -> Int8,
        feedback -> Nullable<Text>,
        feedbackformat -> Nullable<Int2>,
        timemodified -> Int8,
    }
}

table! {
    mdl_groupings (id) {
        id -> Int8,
        courseid -> Int8,
        name -> Varchar,
        idnumber -> Varchar,
        description -> Nullable<Text>,
        descriptionformat -> Int2,
        configdata -> Nullable<Text>,
        timecreated -> Int8,
        timemodified -> Int8,
    }
}

table! {
    mdl_groupings_groups (id) {
        id -> Int8,
        groupingid -> Int8,
        groupid -> Int8,
        timeadded -> Int8,
    }
}

table! {
    mdl_groups (id) {
        id -> Int8,
        courseid -> Int8,
        idnumber -> Varchar,
        name -> Varchar,
        description -> Nullable<Text>,
        descriptionformat -> Int2,
        enrolmentkey -> Nullable<Varchar>,
        picture -> Int8,
        hidepicture -> Int2,
        timecreated -> Int8,
        timemodified -> Int8,
    }
}

table! {
    mdl_groups_members (id) {
        id -> Int8,
        groupid -> Int8,
        userid -> Int8,
        timeadded -> Int8,
        component -> Varchar,
        itemid -> Int8,
    }
}

table! {
    mdl_imscp (id) {
        id -> Int8,
        course -> Int8,
        name -> Varchar,
        intro -> Nullable<Text>,
        introformat -> Int2,
        revision -> Int8,
        keepold -> Int8,
        structure -> Nullable<Text>,
        timemodified -> Int8,
    }
}

table! {
    mdl_label (id) {
        id -> Int8,
        course -> Int8,
        name -> Varchar,
        intro -> Text,
        introformat -> Nullable<Int2>,
        timemodified -> Int8,
    }
}

table! {
    mdl_lesson (id) {
        id -> Int8,
        course -> Int8,
        name -> Varchar,
        practice -> Int2,
        modattempts -> Int2,
        usepassword -> Int2,
        password -> Varchar,
        dependency -> Int8,
        conditions -> Text,
        grade -> Int2,
        custom -> Int2,
        ongoing -> Int2,
        usemaxgrade -> Int2,
        maxanswers -> Int2,
        maxattempts -> Int2,
        review -> Int2,
        nextpagedefault -> Int2,
        feedback -> Int2,
        minquestions -> Int2,
        maxpages -> Int2,
        timed -> Int2,
        maxtime -> Int8,
        retake -> Int2,
        activitylink -> Int8,
        mediafile -> Varchar,
        mediaheight -> Int8,
        mediawidth -> Int8,
        mediaclose -> Int2,
        slideshow -> Int2,
        width -> Int8,
        height -> Int8,
        bgcolor -> Varchar,
        displayleft -> Int2,
        displayleftif -> Int2,
        progressbar -> Int2,
        highscores -> Int2,
        maxhighscores -> Int8,
        available -> Int8,
        deadline -> Int8,
        timemodified -> Int8,
    }
}

table! {
    mdl_lesson_answers (id) {
        id -> Int8,
        lessonid -> Int8,
        pageid -> Int8,
        jumpto -> Int8,
        grade -> Int2,
        score -> Int8,
        flags -> Int2,
        timecreated -> Int8,
        timemodified -> Int8,
        answer -> Nullable<Text>,
        answerformat -> Int2,
        response -> Nullable<Text>,
        responseformat -> Int2,
    }
}

table! {
    mdl_lesson_attempts (id) {
        id -> Int8,
        lessonid -> Int8,
        pageid -> Int8,
        userid -> Int8,
        answerid -> Int8,
        retry -> Int2,
        correct -> Int8,
        useranswer -> Nullable<Text>,
        timeseen -> Int8,
    }
}

table! {
    mdl_lesson_branch (id) {
        id -> Int8,
        lessonid -> Int8,
        userid -> Int8,
        pageid -> Int8,
        retry -> Int8,
        flag -> Int2,
        timeseen -> Int8,
    }
}

table! {
    mdl_lesson_grades (id) {
        id -> Int8,
        lessonid -> Int8,
        userid -> Int8,
        grade -> Float8,
        late -> Int2,
        completed -> Int8,
    }
}

table! {
    mdl_lesson_high_scores (id) {
        id -> Int8,
        lessonid -> Int8,
        userid -> Int8,
        gradeid -> Int8,
        nickname -> Varchar,
    }
}

table! {
    mdl_lesson_pages (id) {
        id -> Int8,
        lessonid -> Int8,
        prevpageid -> Int8,
        nextpageid -> Int8,
        qtype -> Int2,
        qoption -> Int2,
        layout -> Int2,
        display -> Int2,
        timecreated -> Int8,
        timemodified -> Int8,
        title -> Varchar,
        contents -> Text,
        contentsformat -> Int2,
    }
}

table! {
    mdl_lesson_timer (id) {
        id -> Int8,
        lessonid -> Int8,
        userid -> Int8,
        starttime -> Int8,
        lessontime -> Int8,
    }
}

table! {
    mdl_license (id) {
        id -> Int8,
        shortname -> Nullable<Varchar>,
        fullname -> Nullable<Text>,
        source -> Nullable<Varchar>,
        enabled -> Int2,
        version -> Int8,
    }
}

table! {
    mdl_log (id) {
        id -> Int8,
        time -> Int8,
        userid -> Int8,
        ip -> Varchar,
        course -> Int8,
        module -> Varchar,
        cmid -> Int8,
        action -> Varchar,
        url -> Varchar,
        info -> Varchar,
    }
}

table! {
    mdl_log_display (id) {
        id -> Int8,
        module -> Varchar,
        action -> Varchar,
        mtable -> Varchar,
        field -> Varchar,
        component -> Varchar,
    }
}

table! {
    mdl_log_queries (id) {
        id -> Int8,
        qtype -> Int4,
        sqltext -> Text,
        sqlparams -> Nullable<Text>,
        error -> Int4,
        info -> Nullable<Text>,
        backtrace -> Nullable<Text>,
        exectime -> Numeric,
        timelogged -> Int8,
    }
}

table! {
    mdl_lti (id) {
        id -> Int8,
        course -> Int8,
        name -> Varchar,
        intro -> Nullable<Text>,
        introformat -> Nullable<Int2>,
        timecreated -> Int8,
        timemodified -> Int8,
        typeid -> Nullable<Int8>,
        toolurl -> Text,
        securetoolurl -> Nullable<Text>,
        instructorchoicesendname -> Nullable<Int2>,
        instructorchoicesendemailaddr -> Nullable<Int2>,
        instructorchoiceallowroster -> Nullable<Int2>,
        instructorchoiceallowsetting -> Nullable<Int2>,
        instructorcustomparameters -> Nullable<Varchar>,
        instructorchoiceacceptgrades -> Nullable<Int2>,
        grade -> Numeric,
        launchcontainer -> Int2,
        resourcekey -> Nullable<Varchar>,
        password -> Nullable<Varchar>,
        debuglaunch -> Int2,
        showtitlelaunch -> Int2,
        showdescriptionlaunch -> Int2,
        servicesalt -> Nullable<Varchar>,
        icon -> Nullable<Text>,
        secureicon -> Nullable<Text>,
    }
}

table! {
    mdl_lti_submission (id) {
        id -> Int8,
        ltiid -> Int8,
        userid -> Int8,
        datesubmitted -> Int8,
        dateupdated -> Int8,
        gradepercent -> Numeric,
        originalgrade -> Numeric,
        launchid -> Int8,
        state -> Int2,
    }
}

table! {
    mdl_lti_types (id) {
        id -> Int8,
        name -> Varchar,
        baseurl -> Text,
        tooldomain -> Varchar,
        state -> Int2,
        course -> Int8,
        coursevisible -> Int2,
        createdby -> Int8,
        timecreated -> Int8,
        timemodified -> Int8,
    }
}

table! {
    mdl_lti_types_config (id) {
        id -> Int8,
        typeid -> Int8,
        name -> Varchar,
        value -> Varchar,
    }
}

table! {
    mdl_message (id) {
        id -> Int8,
        useridfrom -> Int8,
        useridto -> Int8,
        subject -> Nullable<Text>,
        fullmessage -> Nullable<Text>,
        fullmessageformat -> Nullable<Int2>,
        fullmessagehtml -> Nullable<Text>,
        smallmessage -> Nullable<Text>,
        notification -> Nullable<Int2>,
        contexturl -> Nullable<Text>,
        contexturlname -> Nullable<Text>,
        timecreated -> Int8,
    }
}

table! {
    mdl_message_contacts (id) {
        id -> Int8,
        userid -> Int8,
        contactid -> Int8,
        blocked -> Int2,
    }
}

table! {
    mdl_message_processors (id) {
        id -> Int8,
        name -> Varchar,
        enabled -> Int2,
    }
}

table! {
    mdl_message_providers (id) {
        id -> Int8,
        name -> Varchar,
        component -> Varchar,
        capability -> Nullable<Varchar>,
    }
}

table! {
    mdl_message_read (id) {
        id -> Int8,
        useridfrom -> Int8,
        useridto -> Int8,
        subject -> Nullable<Text>,
        fullmessage -> Nullable<Text>,
        fullmessageformat -> Nullable<Int2>,
        fullmessagehtml -> Nullable<Text>,
        smallmessage -> Nullable<Text>,
        notification -> Nullable<Int2>,
        contexturl -> Nullable<Text>,
        contexturlname -> Nullable<Text>,
        timecreated -> Int8,
        timeread -> Int8,
    }
}

table! {
    mdl_message_working (id) {
        id -> Int8,
        unreadmessageid -> Int8,
        processorid -> Int8,
    }
}

table! {
    mdl_mnet_application (id) {
        id -> Int8,
        name -> Varchar,
        display_name -> Varchar,
        xmlrpc_server_url -> Varchar,
        sso_land_url -> Varchar,
        sso_jump_url -> Varchar,
    }
}

table! {
    mdl_mnet_host (id) {
        id -> Int8,
        deleted -> Int2,
        wwwroot -> Varchar,
        ip_address -> Varchar,
        name -> Varchar,
        public_key -> Text,
        public_key_expires -> Int8,
        transport -> Int2,
        portno -> Int4,
        last_connect_time -> Int8,
        last_log_id -> Int8,
        force_theme -> Int2,
        theme -> Nullable<Varchar>,
        applicationid -> Int8,
    }
}

table! {
    mdl_mnet_host2service (id) {
        id -> Int8,
        hostid -> Int8,
        serviceid -> Int8,
        publish -> Int2,
        subscribe -> Int2,
    }
}

table! {
    mdl_mnet_log (id) {
        id -> Int8,
        hostid -> Int8,
        remoteid -> Int8,
        time -> Int8,
        userid -> Int8,
        ip -> Varchar,
        course -> Int8,
        coursename -> Varchar,
        module -> Varchar,
        cmid -> Int8,
        action -> Varchar,
        url -> Varchar,
        info -> Varchar,
    }
}

table! {
    mdl_mnet_remote_rpc (id) {
        id -> Int8,
        functionname -> Varchar,
        xmlrpcpath -> Varchar,
        plugintype -> Varchar,
        pluginname -> Varchar,
        enabled -> Int2,
    }
}

table! {
    mdl_mnet_remote_service2rpc (id) {
        id -> Int8,
        serviceid -> Int8,
        rpcid -> Int8,
    }
}

table! {
    mdl_mnet_rpc (id) {
        id -> Int8,
        functionname -> Varchar,
        xmlrpcpath -> Varchar,
        plugintype -> Varchar,
        pluginname -> Varchar,
        enabled -> Int2,
        help -> Text,
        profile -> Text,
        filename -> Varchar,
        classname -> Nullable<Varchar>,
        #[sql_name = "static"]
        static_ -> Nullable<Int2>,
    }
}

table! {
    mdl_mnet_service (id) {
        id -> Int8,
        name -> Varchar,
        description -> Varchar,
        apiversion -> Varchar,
        offer -> Int2,
    }
}

table! {
    mdl_mnet_service2rpc (id) {
        id -> Int8,
        serviceid -> Int8,
        rpcid -> Int8,
    }
}

table! {
    mdl_mnetservice_enrol_courses (id) {
        id -> Int8,
        hostid -> Int8,
        remoteid -> Int8,
        categoryid -> Int8,
        categoryname -> Varchar,
        sortorder -> Int8,
        fullname -> Varchar,
        shortname -> Varchar,
        idnumber -> Varchar,
        summary -> Text,
        summaryformat -> Nullable<Int2>,
        startdate -> Int8,
        roleid -> Int8,
        rolename -> Varchar,
    }
}

table! {
    mdl_mnetservice_enrol_enrolments (id) {
        id -> Int8,
        hostid -> Int8,
        userid -> Int8,
        remotecourseid -> Int8,
        rolename -> Varchar,
        enroltime -> Int8,
        enroltype -> Varchar,
    }
}

table! {
    mdl_mnet_session (id) {
        id -> Int8,
        userid -> Int8,
        username -> Varchar,
        token -> Varchar,
        mnethostid -> Int8,
        useragent -> Varchar,
        confirm_timeout -> Int8,
        session_id -> Varchar,
        expires -> Int8,
    }
}

table! {
    mdl_mnet_sso_access_control (id) {
        id -> Int8,
        username -> Varchar,
        mnet_host_id -> Int8,
        accessctrl -> Varchar,
    }
}

table! {
    mdl_modules (id) {
        id -> Int8,
        name -> Varchar,
        version -> Int8,
        cron -> Int8,
        lastcron -> Int8,
        search -> Varchar,
        visible -> Int2,
    }
}

table! {
    mdl_my_pages (id) {
        id -> Int8,
        userid -> Nullable<Int8>,
        name -> Varchar,
        private -> Int2,
        sortorder -> Int4,
    }
}

table! {
    mdl_page (id) {
        id -> Int8,
        course -> Int8,
        name -> Varchar,
        intro -> Nullable<Text>,
        introformat -> Int2,
        content -> Nullable<Text>,
        contentformat -> Int2,
        legacyfiles -> Int2,
        legacyfileslast -> Nullable<Int8>,
        display -> Int2,
        displayoptions -> Nullable<Text>,
        revision -> Int8,
        timemodified -> Int8,
    }
}

table! {
    mdl_plagiarism_programming (id) {
        id -> Int8,
        cmid -> Int8,
        scandate -> Nullable<Int8>,
        jplag -> Nullable<Int2>,
        moss -> Nullable<Int2>,
        language -> Varchar,
        auto_publish -> Nullable<Int2>,
        notification -> Nullable<Varchar>,
        starttime -> Nullable<Int8>,
        latestscan -> Nullable<Int8>,
        notification_text -> Nullable<Text>,
    }
}

table! {
    mdl_plagiarism_programming_cours (id) {
        id -> Int8,
        course -> Int8,
    }
}

table! {
    mdl_plagiarism_programming_date (id) {
        id -> Int8,
        scan_date -> Int8,
        finished -> Int2,
        settingid -> Int8,
    }
}

table! {
    mdl_plagiarism_programming_jplag (id) {
        id -> Int8,
        submissionid -> Nullable<Varchar>,
        status -> Nullable<Varchar>,
        directory -> Nullable<Varchar>,
        message -> Nullable<Varchar>,
        settingid -> Int8,
        progress -> Nullable<Int2>,
        token -> Nullable<Varchar>,
        error_detail -> Nullable<Text>,
    }
}

table! {
    mdl_plagiarism_programming_moss (id) {
        id -> Int8,
        settingid -> Int8,
        resultlink -> Nullable<Varchar>,
        status -> Nullable<Varchar>,
        message -> Nullable<Text>,
        progress -> Nullable<Int2>,
        token -> Nullable<Varchar>,
        error_detail -> Nullable<Text>,
    }
}

table! {
    mdl_plagiarism_programming_reslt (id) {
        id -> Int8,
        student1_id -> Int8,
        student2_id -> Int8,
        additional_codefile_name -> Nullable<Varchar>,
        similarity1 -> Nullable<Numeric>,
        similarity2 -> Nullable<Numeric>,
        comparison -> Nullable<Varchar>,
        comments -> Nullable<Varchar>,
        reportid -> Int8,
        mark -> Nullable<Varchar>,
    }
}

table! {
    mdl_plagiarism_programming_rpt (id) {
        id -> Int8,
        cmid -> Int8,
        time_created -> Int8,
        version -> Int8,
        detector -> Nullable<Varchar>,
    }
}

table! {
    mdl_plagiarism_turnitin_config (id) {
        id -> Int8,
        cm -> Int8,
        name -> Varchar,
        value -> Varchar,
    }
}

table! {
    mdl_plagiarism_turnitin_files (id) {
        id -> Int8,
        cm -> Int8,
        userid -> Int8,
        identifier -> Varchar,
        externalid -> Nullable<Varchar>,
        externalstatus -> Nullable<Int8>,
        statuscode -> Nullable<Varchar>,
        similarityscore -> Nullable<Int8>,
        attempt -> Int4,
        apimd5 -> Nullable<Varchar>,
        legacyteacher -> Int2,
        transmatch -> Nullable<Int8>,
        lastmodified -> Nullable<Int8>,
        grade -> Nullable<Int8>,
        submissiontype -> Nullable<Text>,
        orcapable -> Nullable<Int8>,
        errorcode -> Nullable<Int8>,
        errormsg -> Nullable<Text>,
        submitter -> Nullable<Int8>,
        student_read -> Nullable<Int8>,
    }
}

table! {
    mdl_portfolio_instance (id) {
        id -> Int8,
        plugin -> Varchar,
        name -> Varchar,
        visible -> Int2,
    }
}

table! {
    mdl_portfolio_instance_config (id) {
        id -> Int8,
        instance -> Int8,
        name -> Varchar,
        value -> Nullable<Text>,
    }
}

table! {
    mdl_portfolio_instance_user (id) {
        id -> Int8,
        instance -> Int8,
        userid -> Int8,
        name -> Varchar,
        value -> Nullable<Text>,
    }
}

table! {
    mdl_portfolio_log (id) {
        id -> Int8,
        userid -> Int8,
        time -> Int8,
        portfolio -> Int8,
        caller_class -> Varchar,
        caller_file -> Varchar,
        caller_component -> Nullable<Varchar>,
        caller_sha1 -> Varchar,
        tempdataid -> Int8,
        returnurl -> Varchar,
        continueurl -> Varchar,
    }
}

table! {
    mdl_portfolio_mahara_queue (id) {
        id -> Int8,
        transferid -> Int8,
        token -> Varchar,
    }
}

table! {
    mdl_portfolio_tempdata (id) {
        id -> Int8,
        data -> Nullable<Text>,
        expirytime -> Int8,
        userid -> Int8,
        instance -> Nullable<Int8>,
    }
}

table! {
    mdl_post (id) {
        id -> Int8,
        module -> Varchar,
        userid -> Int8,
        courseid -> Int8,
        groupid -> Int8,
        moduleid -> Int8,
        coursemoduleid -> Int8,
        subject -> Varchar,
        summary -> Nullable<Text>,
        content -> Nullable<Text>,
        uniquehash -> Varchar,
        rating -> Int8,
        format -> Int8,
        summaryformat -> Int2,
        attachment -> Nullable<Varchar>,
        publishstate -> Varchar,
        lastmodified -> Int8,
        created -> Int8,
        usermodified -> Nullable<Int8>,
    }
}

table! {
    mdl_profiling (id) {
        id -> Int8,
        runid -> Varchar,
        url -> Varchar,
        data -> Text,
        totalexecutiontime -> Int8,
        totalcputime -> Int8,
        totalcalls -> Int8,
        totalmemory -> Int8,
        runreference -> Int2,
        runcomment -> Varchar,
        timecreated -> Int8,
    }
}

table! {
    mdl_qtype_essay_options (id) {
        id -> Int8,
        questionid -> Int8,
        responseformat -> Varchar,
        responsefieldlines -> Int2,
        attachments -> Int2,
        graderinfo -> Nullable<Text>,
        graderinfoformat -> Int2,
    }
}

/* Empty
table! {
    mdl_question (id) {
        id -> Int8,
        category -> Int8,
        parent -> Int8,
        name -> Varchar,
        questiontext -> Text,
        questiontextformat -> Int2,
        generalfeedback -> Text,
        generalfeedbackformat -> Int2,
        defaultmark -> Numeric,
        penalty -> Numeric,
        qtype -> Varchar,
        length -> Int8,
        stamp -> Varchar,
        version -> Varchar,
        hidden -> Int2,
        timecreated -> Int8,
        timemodified -> Int8,
        createdby -> Nullable<Int8>,
        modifiedby -> Nullable<Int8>,
    }
}

/ Empty
table! {
    mdl_question_answers (id) {
        id -> Int8,
        question -> Int8,
        answer -> Text,
        answerformat -> Int2,
        fraction -> Numeric,
        feedback -> Text,
        feedbackformat -> Int2,
    }
}

/ Empty
table! {
    mdl_question_attempts (id) {
        id -> Int8,
        questionusageid -> Int8,
        slot -> Int8,
        behaviour -> Varchar,
        questionid -> Int8,
        variant -> Int8,
        maxmark -> Numeric,
        minfraction -> Numeric,
        flagged -> Int2,
        questionsummary -> Nullable<Text>,
        rightanswer -> Nullable<Text>,
        responsesummary -> Nullable<Text>,
        timemodified -> Int8,
    }
}

/ Empty
table! {
    mdl_question_attempt_step_data (id) {
        id -> Int8,
        attemptstepid -> Int8,
        name -> Varchar,
        value -> Nullable<Text>,
    }
}

/ Empty
table! {
    mdl_question_attempt_steps (id) {
        id -> Int8,
        questionattemptid -> Int8,
        sequencenumber -> Int8,
        state -> Varchar,
        fraction -> Nullable<Numeric>,
        timecreated -> Int8,
        userid -> Nullable<Int8>,
    }
}

/ Empty
table! {
    mdl_question_calculated (id) {
        id -> Int8,
        question -> Int8,
        answer -> Int8,
        tolerance -> Varchar,
        tolerancetype -> Int8,
        correctanswerlength -> Int8,
        correctanswerformat -> Int8,
    }
}

/ Empty
table! {
    mdl_question_calculated_options (id) {
        id -> Int8,
        question -> Int8,
        synchronize -> Int2,
        single -> Int2,
        shuffleanswers -> Int2,
        correctfeedback -> Nullable<Text>,
        correctfeedbackformat -> Int2,
        partiallycorrectfeedback -> Nullable<Text>,
        partiallycorrectfeedbackformat -> Int2,
        incorrectfeedback -> Nullable<Text>,
        incorrectfeedbackformat -> Int2,
        answernumbering -> Varchar,
        shownumcorrect -> Int2,
    }
}

/ Empty
table! {
    mdl_question_categories (id) {
        id -> Int8,
        name -> Varchar,
        contextid -> Int8,
        info -> Text,
        infoformat -> Int2,
        stamp -> Varchar,
        parent -> Int8,
        sortorder -> Int8,
    }
}

/ Empty
table! {
    mdl_question_dataset_definitions (id) {
        id -> Int8,
        category -> Int8,
        name -> Varchar,
        #[sql_name = "type"]
        type_ -> Int8,
        options -> Varchar,
        itemcount -> Int8,
    }
}

/ Empty
table! {
    mdl_question_dataset_items (id) {
        id -> Int8,
        definition -> Int8,
        itemnumber -> Int8,
        value -> Varchar,
    }
}

/ Empty
table! {
    mdl_question_datasets (id) {
        id -> Int8,
        question -> Int8,
        datasetdefinition -> Int8,
    }
}

/ Empty
table! {
    mdl_question_hints (id) {
        id -> Int8,
        questionid -> Int8,
        hint -> Text,
        hintformat -> Int2,
        shownumcorrect -> Nullable<Int2>,
        clearwrong -> Nullable<Int2>,
        options -> Nullable<Varchar>,
    }
}

/ Empty
table! {
    mdl_question_match (id) {
        id -> Int8,
        question -> Int8,
        subquestions -> Varchar,
        shuffleanswers -> Int2,
        correctfeedback -> Text,
        correctfeedbackformat -> Int2,
        partiallycorrectfeedback -> Text,
        partiallycorrectfeedbackformat -> Int2,
        incorrectfeedback -> Text,
        incorrectfeedbackformat -> Int2,
        shownumcorrect -> Int2,
    }
}

/ Empty
table! {
    mdl_question_match_sub (id) {
        id -> Int8,
        code -> Int8,
        question -> Int8,
        questiontext -> Text,
        questiontextformat -> Int2,
        answertext -> Varchar,
    }
}

/ Empty
table! {
    mdl_question_multianswer (id) {
        id -> Int8,
        question -> Int8,
        sequence -> Text,
    }
}

/ Empty
table! {
    mdl_question_multichoice (id) {
        id -> Int8,
        question -> Int8,
        layout -> Int2,
        answers -> Varchar,
        single -> Int2,
        shuffleanswers -> Int2,
        correctfeedback -> Text,
        correctfeedbackformat -> Int2,
        partiallycorrectfeedback -> Text,
        partiallycorrectfeedbackformat -> Int2,
        incorrectfeedback -> Text,
        incorrectfeedbackformat -> Int2,
        answernumbering -> Varchar,
        shownumcorrect -> Int2,
    }
}

/ Empty
table! {
    mdl_question_numerical (id) {
        id -> Int8,
        question -> Int8,
        answer -> Int8,
        tolerance -> Varchar,
    }
}

/ Empty
table! {
    mdl_question_numerical_options (id) {
        id -> Int8,
        question -> Int8,
        showunits -> Int2,
        unitsleft -> Int2,
        unitgradingtype -> Int2,
        unitpenalty -> Numeric,
    }
}

/ Empty
table! {
    mdl_question_numerical_units (id) {
        id -> Int8,
        question -> Int8,
        multiplier -> Numeric,
        unit -> Varchar,
    }
}

/ Empty
table! {
    mdl_question_randomsamatch (id) {
        id -> Int8,
        question -> Int8,
        choose -> Int8,
    }
}

/ Empty
table! {
    mdl_question_sessions (id) {
        id -> Int8,
        attemptid -> Int8,
        questionid -> Int8,
        newest -> Int8,
        newgraded -> Int8,
        sumpenalty -> Numeric,
        manualcomment -> Text,
        manualcommentformat -> Int2,
        flagged -> Int2,
    }
}

/ Empty
table! {
    mdl_question_shortanswer (id) {
        id -> Int8,
        question -> Int8,
        answers -> Varchar,
        usecase -> Int2,
    }
}

/ Empty
table! {
    mdl_question_states (id) {
        id -> Int8,
        attempt -> Int8,
        question -> Int8,
        seq_number -> Int4,
        answer -> Text,
        timestamp -> Int8,
        event -> Int2,
        grade -> Numeric,
        raw_grade -> Numeric,
        penalty -> Numeric,
    }
}

/ Empty
table! {
    mdl_question_truefalse (id) {
        id -> Int8,
        question -> Int8,
        trueanswer -> Int8,
        falseanswer -> Int8,
    }
}

/ Empty
table! {
    mdl_question_usages (id) {
        id -> Int8,
        contextid -> Int8,
        component -> Varchar,
        preferredbehaviour -> Varchar,
    }
}

/ Empty
table! {
    mdl_quiz (id) {
        id -> Int8,
        course -> Int8,
        name -> Varchar,
        intro -> Text,
        introformat -> Int2,
        timeopen -> Int8,
        timeclose -> Int8,
        timelimit -> Int8,
        overduehandling -> Varchar,
        graceperiod -> Int8,
        preferredbehaviour -> Varchar,
        attempts -> Int4,
        attemptonlast -> Int2,
        grademethod -> Int2,
        decimalpoints -> Int2,
        questiondecimalpoints -> Int2,
        reviewattempt -> Int4,
        reviewcorrectness -> Int4,
        reviewmarks -> Int4,
        reviewspecificfeedback -> Int4,
        reviewgeneralfeedback -> Int4,
        reviewrightanswer -> Int4,
        reviewoverallfeedback -> Int4,
        questionsperpage -> Int8,
        navmethod -> Varchar,
        shufflequestions -> Int2,
        shuffleanswers -> Int2,
        questions -> Text,
        sumgrades -> Numeric,
        grade -> Numeric,
        timecreated -> Int8,
        timemodified -> Int8,
        password -> Varchar,
        subnet -> Varchar,
        browsersecurity -> Varchar,
        delay1 -> Int8,
        delay2 -> Int8,
        showuserpicture -> Int2,
        showblocks -> Int2,
    }
}

/ Empty
table! {
    mdl_quiz_attempts (id) {
        id -> Int8,
        quiz -> Int8,
        userid -> Int8,
        attempt -> Int4,
        uniqueid -> Int8,
        layout -> Text,
        currentpage -> Int8,
        preview -> Int2,
        state -> Varchar,
        timestart -> Int8,
        timefinish -> Int8,
        timemodified -> Int8,
        timecheckstate -> Nullable<Int8>,
        sumgrades -> Nullable<Numeric>,
        needsupgradetonewqe -> Int2,
    }
}

/ Empty
table! {
    mdl_quiz_feedback (id) {
        id -> Int8,
        quizid -> Int8,
        feedbacktext -> Text,
        feedbacktextformat -> Int2,
        mingrade -> Numeric,
        maxgrade -> Numeric,
    }
}

/ Empty
table! {
    mdl_quiz_grades (id) {
        id -> Int8,
        quiz -> Int8,
        userid -> Int8,
        grade -> Numeric,
        timemodified -> Int8,
    }
}

/ Empty
table! {
    mdl_quiz_overrides (id) {
        id -> Int8,
        quiz -> Int8,
        groupid -> Nullable<Int8>,
        userid -> Nullable<Int8>,
        timeopen -> Nullable<Int8>,
        timeclose -> Nullable<Int8>,
        timelimit -> Nullable<Int8>,
        attempts -> Nullable<Int4>,
        password -> Nullable<Varchar>,
    }
}

/ Empty
table! {
    mdl_quiz_overview_regrades (id) {
        id -> Int8,
        questionusageid -> Int8,
        slot -> Int8,
        newfraction -> Nullable<Numeric>,
        oldfraction -> Nullable<Numeric>,
        regraded -> Int2,
        timemodified -> Int8,
    }
}

/ Empty
table! {
    mdl_quiz_question_instances (id) {
        id -> Int8,
        quiz -> Int8,
        question -> Int8,
        grade -> Numeric,
    }
}

/ Empty
table! {
    mdl_quiz_question_response_stats (id) {
        id -> Int8,
        quizstatisticsid -> Int8,
        questionid -> Int8,
        subqid -> Varchar,
        aid -> Nullable<Varchar>,
        response -> Nullable<Text>,
        rcount -> Nullable<Int8>,
        credit -> Numeric,
    }
}

/ Empty
table! {
    mdl_quiz_question_statistics (id) {
        id -> Int8,
        quizstatisticsid -> Int8,
        questionid -> Int8,
        slot -> Nullable<Int8>,
        subquestion -> Int2,
        s -> Int8,
        effectiveweight -> Nullable<Numeric>,
        negcovar -> Int2,
        discriminationindex -> Nullable<Numeric>,
        discriminativeefficiency -> Nullable<Numeric>,
        sd -> Nullable<Numeric>,
        facility -> Nullable<Numeric>,
        subquestions -> Nullable<Text>,
        maxmark -> Nullable<Numeric>,
        positions -> Nullable<Text>,
        randomguessscore -> Nullable<Numeric>,
    }
}
 */

table! {
    mdl_quiz_reports (id) {
        id -> Int8,
        name -> Nullable<Varchar>,
        displayorder -> Int8,
        capability -> Nullable<Varchar>,
    }
}

/* Empty
table! {
    mdl_quiz_statistics (id) {
        id -> Int8,
        quizid -> Int8,
        groupid -> Int8,
        allattempts -> Int2,
        timemodified -> Int8,
        firstattemptscount -> Int8,
        allattemptscount -> Int8,
        firstattemptsavg -> Nullable<Numeric>,
        allattemptsavg -> Nullable<Numeric>,
        median -> Nullable<Numeric>,
        standarddeviation -> Nullable<Numeric>,
        skewness -> Nullable<Numeric>,
        kurtosis -> Nullable<Numeric>,
        cic -> Nullable<Numeric>,
        errorratio -> Nullable<Numeric>,
        standarderror -> Nullable<Numeric>,
    }
}
*/

table! {
    mdl_rating (id) {
        id -> Int8,
        contextid -> Int8,
        component -> Varchar,
        ratingarea -> Varchar,
        itemid -> Int8,
        scaleid -> Int8,
        rating -> Int8,
        userid -> Int8,
        timecreated -> Int8,
        timemodified -> Int8,
    }
}

table! {
    mdl_registration_hubs (id) {
        id -> Int8,
        token -> Varchar,
        hubname -> Varchar,
        huburl -> Varchar,
        confirmed -> Int2,
        secret -> Nullable<Varchar>,
    }
}

table! {
    mdl_repository (id) {
        id -> Int8,
        #[sql_name = "type"]
        type_ -> Varchar,
        visible -> Nullable<Int2>,
        sortorder -> Int8,
    }
}

table! {
    mdl_repository_instance_config (id) {
        id -> Int8,
        instanceid -> Int8,
        name -> Varchar,
        value -> Nullable<Text>,
    }
}

table! {
    mdl_repository_instances (id) {
        id -> Int8,
        name -> Varchar,
        typeid -> Int8,
        userid -> Int8,
        contextid -> Int8,
        username -> Nullable<Varchar>,
        password -> Nullable<Varchar>,
        timecreated -> Nullable<Int8>,
        timemodified -> Nullable<Int8>,
        readonly -> Int2,
    }
}

table! {
    mdl_resource (id) {
        id -> Int8,
        course -> Int8,
        name -> Varchar,
        intro -> Nullable<Text>,
        introformat -> Int2,
        tobemigrated -> Int2,
        legacyfiles -> Int2,
        legacyfileslast -> Nullable<Int8>,
        display -> Int2,
        displayoptions -> Nullable<Text>,
        filterfiles -> Int2,
        revision -> Int8,
        timemodified -> Int8,
    }
}

table! {
    mdl_resource_old (id) {
        id -> Int8,
        course -> Int8,
        name -> Varchar,
        #[sql_name = "type"]
        type_ -> Varchar,
        reference -> Varchar,
        intro -> Nullable<Text>,
        introformat -> Int2,
        alltext -> Text,
        popup -> Text,
        options -> Varchar,
        timemodified -> Int8,
        oldid -> Int8,
        cmid -> Nullable<Int8>,
        newmodule -> Nullable<Varchar>,
        newid -> Nullable<Int8>,
        migrated -> Int8,
    }
}

table! {
    mdl_role (id) {
        id -> Int8,
        name -> Varchar,
        shortname -> Varchar,
        description -> Text,
        sortorder -> Int8,
        archetype -> Varchar,
    }
}

table! {
    mdl_role_allow_assign (id) {
        id -> Int8,
        roleid -> Int8,
        allowassign -> Int8,
    }
}

table! {
    mdl_role_allow_override (id) {
        id -> Int8,
        roleid -> Int8,
        allowoverride -> Int8,
    }
}

table! {
    mdl_role_allow_switch (id) {
        id -> Int8,
        roleid -> Int8,
        allowswitch -> Int8,
    }
}

table! {
    mdl_role_assignments (id) {
        id -> Int8,
        roleid -> Int8,
        contextid -> Int8,
        userid -> Int8,
        timemodified -> Int8,
        modifierid -> Int8,
        component -> Varchar,
        itemid -> Int8,
        sortorder -> Int8,
    }
}

table! {
    mdl_role_capabilities (id) {
        id -> Int8,
        contextid -> Int8,
        roleid -> Int8,
        capability -> Varchar,
        permission -> Int8,
        timemodified -> Int8,
        modifierid -> Int8,
    }
}

table! {
    mdl_role_context_levels (id) {
        id -> Int8,
        roleid -> Int8,
        contextlevel -> Int8,
    }
}

table! {
    mdl_role_names (id) {
        id -> Int8,
        roleid -> Int8,
        contextid -> Int8,
        name -> Varchar,
    }
}

table! {
    mdl_role_sortorder (id) {
        id -> Int8,
        userid -> Int8,
        roleid -> Int8,
        contextid -> Int8,
        sortoder -> Int8,
    }
}

table! {
    mdl_scale (id) {
        id -> Int8,
        courseid -> Int8,
        userid -> Int8,
        name -> Varchar,
        scale -> Text,
        description -> Text,
        descriptionformat -> Int2,
        timemodified -> Int8,
    }
}

table! {
    mdl_scale_history (id) {
        id -> Int8,
        action -> Int8,
        oldid -> Int8,
        source -> Nullable<Varchar>,
        timemodified -> Nullable<Int8>,
        loggeduser -> Nullable<Int8>,
        courseid -> Int8,
        userid -> Int8,
        name -> Varchar,
        scale -> Text,
        description -> Text,
    }
}

table! {
    mdl_scorm (id) {
        id -> Int8,
        course -> Int8,
        name -> Varchar,
        scormtype -> Varchar,
        reference -> Varchar,
        intro -> Text,
        introformat -> Int2,
        version -> Varchar,
        maxgrade -> Float8,
        grademethod -> Int2,
        whatgrade -> Int8,
        maxattempt -> Int8,
        forcecompleted -> Int2,
        forcenewattempt -> Int2,
        lastattemptlock -> Int2,
        displayattemptstatus -> Int2,
        displaycoursestructure -> Int2,
        updatefreq -> Int2,
        sha1hash -> Nullable<Varchar>,
        md5hash -> Varchar,
        revision -> Int8,
        launch -> Int8,
        skipview -> Int2,
        hidebrowse -> Int2,
        hidetoc -> Int2,
        hidenav -> Int2,
        auto -> Int2,
        popup -> Int2,
        options -> Varchar,
        width -> Int8,
        height -> Int8,
        timeopen -> Int8,
        timeclose -> Int8,
        timemodified -> Int8,
        completionstatusrequired -> Nullable<Int2>,
        completionscorerequired -> Nullable<Int2>,
    }
}

table! {
    mdl_scorm_aicc_session (id) {
        id -> Int8,
        userid -> Int8,
        scormid -> Int8,
        hacpsession -> Varchar,
        scoid -> Nullable<Int8>,
        scormmode -> Nullable<Varchar>,
        scormstatus -> Nullable<Varchar>,
        attempt -> Nullable<Int8>,
        lessonstatus -> Nullable<Varchar>,
        sessiontime -> Nullable<Varchar>,
        timecreated -> Int8,
        timemodified -> Int8,
    }
}

table! {
    mdl_scorm_scoes (id) {
        id -> Int8,
        scorm -> Int8,
        manifest -> Varchar,
        organization -> Varchar,
        parent -> Varchar,
        identifier -> Varchar,
        launch -> Text,
        scormtype -> Varchar,
        title -> Varchar,
    }
}

table! {
    mdl_scorm_scoes_data (id) {
        id -> Int8,
        scoid -> Int8,
        name -> Varchar,
        value -> Text,
    }
}

table! {
    mdl_scorm_scoes_track (id) {
        id -> Int8,
        userid -> Int8,
        scormid -> Int8,
        scoid -> Int8,
        attempt -> Int8,
        element -> Varchar,
        value -> Text,
        timemodified -> Int8,
    }
}

table! {
    mdl_scorm_seq_mapinfo (id) {
        id -> Int8,
        scoid -> Int8,
        objectiveid -> Int8,
        targetobjectiveid -> Int8,
        readsatisfiedstatus -> Int2,
        readnormalizedmeasure -> Int2,
        writesatisfiedstatus -> Int2,
        writenormalizedmeasure -> Int2,
    }
}

table! {
    mdl_scorm_seq_objective (id) {
        id -> Int8,
        scoid -> Int8,
        primaryobj -> Int2,
        objectiveid -> Varchar,
        satisfiedbymeasure -> Int2,
        minnormalizedmeasure -> Float4,
    }
}

table! {
    mdl_scorm_seq_rolluprule (id) {
        id -> Int8,
        scoid -> Int8,
        childactivityset -> Varchar,
        minimumcount -> Int8,
        minimumpercent -> Float4,
        conditioncombination -> Varchar,
        action -> Varchar,
    }
}

table! {
    mdl_scorm_seq_rolluprulecond (id) {
        id -> Int8,
        scoid -> Int8,
        rollupruleid -> Int8,
        operator -> Varchar,
        cond -> Varchar,
    }
}

table! {
    mdl_scorm_seq_rulecond (id) {
        id -> Int8,
        scoid -> Int8,
        ruleconditionsid -> Int8,
        refrencedobjective -> Varchar,
        measurethreshold -> Float4,
        operator -> Varchar,
        cond -> Varchar,
    }
}

table! {
    mdl_scorm_seq_ruleconds (id) {
        id -> Int8,
        scoid -> Int8,
        conditioncombination -> Varchar,
        ruletype -> Int2,
        action -> Varchar,
    }
}

table! {
    mdl_sessions (id) {
        id -> Int8,
        state -> Int8,
        sid -> Varchar,
        userid -> Int8,
        sessdata -> Nullable<Text>,
        timecreated -> Int8,
        timemodified -> Int8,
        firstip -> Nullable<Varchar>,
        lastip -> Nullable<Varchar>,
    }
}

table! {
    mdl_stats_daily (id) {
        id -> Int8,
        courseid -> Int8,
        timeend -> Int8,
        roleid -> Int8,
        stattype -> Varchar,
        stat1 -> Int8,
        stat2 -> Int8,
    }
}

table! {
    mdl_stats_monthly (id) {
        id -> Int8,
        courseid -> Int8,
        timeend -> Int8,
        roleid -> Int8,
        stattype -> Varchar,
        stat1 -> Int8,
        stat2 -> Int8,
    }
}

table! {
    mdl_stats_user_daily (id) {
        id -> Int8,
        courseid -> Int8,
        userid -> Int8,
        roleid -> Int8,
        timeend -> Int8,
        statsreads -> Int8,
        statswrites -> Int8,
        stattype -> Varchar,
    }
}

table! {
    mdl_stats_user_monthly (id) {
        id -> Int8,
        courseid -> Int8,
        userid -> Int8,
        roleid -> Int8,
        timeend -> Int8,
        statsreads -> Int8,
        statswrites -> Int8,
        stattype -> Varchar,
    }
}

table! {
    mdl_stats_user_weekly (id) {
        id -> Int8,
        courseid -> Int8,
        userid -> Int8,
        roleid -> Int8,
        timeend -> Int8,
        statsreads -> Int8,
        statswrites -> Int8,
        stattype -> Varchar,
    }
}

table! {
    mdl_stats_weekly (id) {
        id -> Int8,
        courseid -> Int8,
        timeend -> Int8,
        roleid -> Int8,
        stattype -> Varchar,
        stat1 -> Int8,
        stat2 -> Int8,
    }
}

table! {
    mdl_survey (id) {
        id -> Int8,
        course -> Int8,
        template -> Int8,
        days -> Int4,
        timecreated -> Int8,
        timemodified -> Int8,
        name -> Varchar,
        intro -> Text,
        introformat -> Int2,
        questions -> Varchar,
    }
}

table! {
    mdl_survey_analysis (id) {
        id -> Int8,
        survey -> Int8,
        userid -> Int8,
        notes -> Text,
    }
}

table! {
    mdl_survey_answers (id) {
        id -> Int8,
        userid -> Int8,
        survey -> Int8,
        question -> Int8,
        time -> Int8,
        answer1 -> Text,
        answer2 -> Text,
    }
}

table! {
    mdl_survey_questions (id) {
        id -> Int8,
        text -> Varchar,
        shorttext -> Varchar,
        multi -> Varchar,
        intro -> Varchar,
        #[sql_name = "type"]
        type_ -> Int2,
        options -> Nullable<Text>,
    }
}

table! {
    mdl_tag (id) {
        id -> Int8,
        userid -> Int8,
        name -> Varchar,
        rawname -> Varchar,
        tagtype -> Nullable<Varchar>,
        description -> Nullable<Text>,
        descriptionformat -> Int2,
        flag -> Nullable<Int2>,
        timemodified -> Nullable<Int8>,
    }
}

table! {
    mdl_tag_correlation (id) {
        id -> Int8,
        tagid -> Int8,
        correlatedtags -> Text,
    }
}

table! {
    mdl_tag_instance (id) {
        id -> Int8,
        tagid -> Int8,
        itemtype -> Varchar,
        itemid -> Int8,
        tiuserid -> Int8,
        ordering -> Nullable<Int8>,
        timemodified -> Int8,
    }
}

table! {
    mdl_temp_enroled_template (id) {
        id -> Int8,
        userid -> Int8,
        courseid -> Int8,
        roleid -> Int8,
    }
}

table! {
    mdl_temp_log_template (id) {
        id -> Int8,
        userid -> Int8,
        course -> Int8,
        action -> Varchar,
    }
}

table! {
    mdl_timezone (id) {
        id -> Int8,
        name -> Varchar,
        year -> Int8,
        tzrule -> Varchar,
        gmtoff -> Int8,
        dstoff -> Int8,
        dst_month -> Int2,
        dst_startday -> Int2,
        dst_weekday -> Int2,
        dst_skipweeks -> Int2,
        dst_time -> Varchar,
        std_month -> Int2,
        std_startday -> Int2,
        std_weekday -> Int2,
        std_skipweeks -> Int2,
        std_time -> Varchar,
    }
}

table! {
    mdl_tool_customlang (id) {
        id -> Int8,
        lang -> Varchar,
        componentid -> Int8,
        stringid -> Varchar,
        original -> Text,
        master -> Nullable<Text>,
        local -> Nullable<Text>,
        timemodified -> Int8,
        timecustomized -> Nullable<Int8>,
        outdated -> Nullable<Int2>,
        modified -> Nullable<Int2>,
    }
}

table! {
    mdl_tool_customlang_components (id) {
        id -> Int8,
        name -> Varchar,
        version -> Nullable<Varchar>,
    }
}

table! {
    mdl_turnitintooltwo (id) {
        id -> Int8,
        course -> Int8,
        #[sql_name = "type"]
        type_ -> Int8,
        name -> Varchar,
        grade -> Int8,
        numparts -> Int8,
        defaultdtstart -> Int8,
        defaultdtdue -> Int8,
        defaultdtpost -> Int8,
        anon -> Nullable<Int2>,
        submitted -> Nullable<Int2>,
        portfolio -> Nullable<Int2>,
        allowlate -> Int2,
        reportgenspeed -> Int2,
        submitpapersto -> Int2,
        spapercheck -> Int2,
        internetcheck -> Int2,
        journalcheck -> Int2,
        maxfilesize -> Nullable<Int8>,
        intro -> Nullable<Text>,
        introformat -> Int2,
        timecreated -> Int8,
        timemodified -> Int8,
        studentreports -> Int8,
        dateformat -> Text,
        usegrademark -> Int8,
        gradedisplay -> Int8,
        autoupdates -> Int8,
        commentedittime -> Int8,
        commentmaxsize -> Int8,
        autosubmission -> Int8,
        shownonsubmission -> Int8,
        excludebiblio -> Int2,
        excludequoted -> Int2,
        excludevalue -> Int4,
        excludetype -> Int2,
        perpage -> Int8,
        erater -> Int8,
        erater_handbook -> Int8,
        erater_dictionary -> Nullable<Varchar>,
        erater_spelling -> Int8,
        erater_grammar -> Int8,
        erater_usage -> Int8,
        erater_mechanics -> Int8,
        erater_style -> Int8,
        transmatch -> Int8,
        rubric -> Int8,
        allownonor -> Int8,
        needs_updating -> Int2,
        institution_check -> Nullable<Int2>,
    }
}

table! {
    mdl_turnitintooltwo_courses (id) {
        id -> Int8,
        courseid -> Int8,
        ownerid -> Int8,
        turnitin_ctl -> Text,
        turnitin_cid -> Int8,
        course_type -> Varchar,
    }
}

table! {
    mdl_turnitintooltwo_parts (id) {
        id -> Int8,
        turnitintooltwoid -> Int8,
        partname -> Text,
        tiiassignid -> Int8,
        dtstart -> Int8,
        dtdue -> Int8,
        dtpost -> Int8,
        maxmarks -> Int8,
        deleted -> Int8,
        migrated -> Int8,
        unanon -> Int2,
        submitted -> Int2,
    }
}

table! {
    mdl_turnitintooltwo_peermarks (id) {
        id -> Int8,
        parent_tii_assign_id -> Int8,
        title -> Nullable<Text>,
        tiiassignid -> Int8,
        dtstart -> Int8,
        dtdue -> Int8,
        dtpost -> Int8,
        maxmarks -> Int8,
        instructions -> Nullable<Text>,
        distributed_reviews -> Int8,
        selected_reviews -> Int8,
        self_review -> Int8,
        non_submitters_review -> Int8,
    }
}

table! {
    mdl_turnitintooltwo_submissions (id) {
        id -> Int8,
        userid -> Int8,
        turnitintooltwoid -> Int8,
        submission_part -> Nullable<Int8>,
        submission_title -> Text,
        submission_type -> Int2,
        submission_filename -> Nullable<Text>,
        submission_objectid -> Nullable<Int8>,
        submission_score -> Nullable<Int8>,
        submission_grade -> Nullable<Int8>,
        submission_gmimaged -> Nullable<Int8>,
        submission_status -> Nullable<Text>,
        submission_queued -> Nullable<Int8>,
        submission_attempts -> Int8,
        submission_modified -> Int8,
        submission_parent -> Nullable<Int8>,
        submission_nmuserid -> Nullable<Varchar>,
        submission_nmfirstname -> Nullable<Text>,
        submission_nmlastname -> Nullable<Text>,
        submission_unanon -> Int2,
        submission_unanonreason -> Nullable<Text>,
        submission_transmatch -> Int8,
        submission_acceptnothing -> Int8,
        submission_orcapable -> Int8,
    }
}

table! {
    mdl_turnitintooltwo_users (id) {
        id -> Int8,
        userid -> Int8,
        turnitin_uid -> Int8,
        turnitin_utp -> Int8,
        instructor_defaults -> Nullable<Text>,
        instructor_rubrics -> Nullable<Text>,
        user_agreement_accepted -> Int2,
    }
}

table! {
    mdl_upgrade_log (id) {
        id -> Int8,
        #[sql_name = "type"]
        type_ -> Int8,
        plugin -> Nullable<Varchar>,
        version -> Nullable<Varchar>,
        targetversion -> Nullable<Varchar>,
        info -> Varchar,
        details -> Nullable<Text>,
        backtrace -> Nullable<Text>,
        userid -> Int8,
        timemodified -> Int8,
    }
}

table! {
    mdl_url (id) {
        id -> Int8,
        course -> Int8,
        name -> Varchar,
        intro -> Nullable<Text>,
        introformat -> Int2,
        externalurl -> Text,
        display -> Int2,
        displayoptions -> Nullable<Text>,
        parameters -> Nullable<Text>,
        timemodified -> Int8,
    }
}

table! {
    mdl_user (id) {
        id -> Int8,
        auth -> Varchar,
        confirmed -> Int2,
        policyagreed -> Int2,
        deleted -> Int2,
        suspended -> Int2,
        mnethostid -> Int8,
        username -> Varchar,
        password -> Varchar,
        idnumber -> Varchar,
        firstname -> Varchar,
        lastname -> Varchar,
        email -> Varchar,
        emailstop -> Int2,
        icq -> Varchar,
        skype -> Varchar,
        yahoo -> Varchar,
        aim -> Varchar,
        msn -> Varchar,
        phone1 -> Varchar,
        phone2 -> Varchar,
        institution -> Varchar,
        department -> Varchar,
        address -> Varchar,
        city -> Varchar,
        country -> Varchar,
        lang -> Varchar,
        theme -> Varchar,
        timezone -> Varchar,
        firstaccess -> Int8,
        lastaccess -> Int8,
        lastlogin -> Int8,
        currentlogin -> Int8,
        lastip -> Varchar,
        secret -> Varchar,
        picture -> Int8,
        url -> Varchar,
        description -> Nullable<Text>,
        descriptionformat -> Int2,
        mailformat -> Int2,
        maildigest -> Int2,
        maildisplay -> Int2,
        htmleditor -> Int2,
        autosubscribe -> Int2,
        trackforums -> Int2,
        timecreated -> Int8,
        timemodified -> Int8,
        trustbitmask -> Int8,
        imagealt -> Nullable<Varchar>,
    }
}

table! {
    mdl_user_enrolments (id) {
        id -> Int8,
        status -> Int8,
        enrolid -> Int8,
        userid -> Int8,
        timestart -> Int8,
        timeend -> Int8,
        modifierid -> Int8,
        timecreated -> Int8,
        timemodified -> Int8,
    }
}

table! {
    mdl_user_info_category (id) {
        id -> Int8,
        name -> Varchar,
        sortorder -> Int8,
    }
}

table! {
    mdl_user_info_data (id) {
        id -> Int8,
        userid -> Int8,
        fieldid -> Int8,
        data -> Text,
        dataformat -> Int2,
    }
}

table! {
    mdl_user_info_field (id) {
        id -> Int8,
        shortname -> Varchar,
        name -> Text,
        datatype -> Varchar,
        description -> Nullable<Text>,
        descriptionformat -> Int2,
        categoryid -> Int8,
        sortorder -> Int8,
        required -> Int2,
        locked -> Int2,
        visible -> Int2,
        forceunique -> Int2,
        signup -> Int2,
        defaultdata -> Nullable<Text>,
        defaultdataformat -> Int2,
        param1 -> Nullable<Text>,
        param2 -> Nullable<Text>,
        param3 -> Nullable<Text>,
        param4 -> Nullable<Text>,
        param5 -> Nullable<Text>,
    }
}

table! {
    mdl_user_lastaccess (id) {
        id -> Int8,
        userid -> Int8,
        courseid -> Int8,
        timeaccess -> Int8,
    }
}

table! {
    mdl_user_preferences (id) {
        id -> Int8,
        userid -> Int8,
        name -> Varchar,
        value -> Varchar,
    }
}

table! {
    mdl_user_private_key (id) {
        id -> Int8,
        script -> Varchar,
        value -> Varchar,
        userid -> Int8,
        instance -> Nullable<Int8>,
        iprestriction -> Nullable<Varchar>,
        validuntil -> Nullable<Int8>,
        timecreated -> Nullable<Int8>,
    }
}

table! {
    mdl_webdav_locks (id) {
        id -> Int8,
        token -> Varchar,
        path -> Varchar,
        expiry -> Int8,
        userid -> Int8,
        recursive -> Int2,
        exclusivelock -> Int2,
        created -> Int8,
        modified -> Int8,
        owner -> Nullable<Varchar>,
    }
}

table! {
    mdl_wiki (id) {
        id -> Int8,
        course -> Int8,
        name -> Varchar,
        intro -> Nullable<Text>,
        introformat -> Int2,
        timecreated -> Int8,
        timemodified -> Int8,
        firstpagetitle -> Varchar,
        wikimode -> Varchar,
        defaultformat -> Varchar,
        forceformat -> Int2,
        editbegin -> Int8,
        editend -> Nullable<Int8>,
    }
}

table! {
    mdl_wiki_links (id) {
        id -> Int8,
        subwikiid -> Int8,
        frompageid -> Int8,
        topageid -> Int8,
        tomissingpage -> Nullable<Varchar>,
    }
}

table! {
    mdl_wiki_locks (id) {
        id -> Int8,
        pageid -> Int8,
        sectionname -> Nullable<Varchar>,
        userid -> Int8,
        lockedat -> Int8,
    }
}

table! {
    mdl_wiki_pages (id) {
        id -> Int8,
        subwikiid -> Int8,
        title -> Varchar,
        cachedcontent -> Text,
        timecreated -> Int8,
        timemodified -> Int8,
        timerendered -> Int8,
        userid -> Int8,
        pageviews -> Int8,
        readonly -> Int2,
    }
}

table! {
    mdl_wiki_subwikis (id) {
        id -> Int8,
        wikiid -> Int8,
        groupid -> Int8,
        userid -> Int8,
    }
}

table! {
    mdl_wiki_synonyms (id) {
        id -> Int8,
        subwikiid -> Int8,
        pageid -> Int8,
        pagesynonym -> Varchar,
    }
}

table! {
    mdl_wiki_versions (id) {
        id -> Int8,
        pageid -> Int8,
        content -> Text,
        contentformat -> Varchar,
        version -> Int4,
        timecreated -> Int8,
        userid -> Int8,
    }
}

table! {
    mdl_workshop (id) {
        id -> Int8,
        course -> Int8,
        name -> Varchar,
        intro -> Nullable<Text>,
        introformat -> Int2,
        instructauthors -> Nullable<Text>,
        instructauthorsformat -> Int2,
        instructreviewers -> Nullable<Text>,
        instructreviewersformat -> Int2,
        timemodified -> Int8,
        phase -> Nullable<Int2>,
        useexamples -> Nullable<Int2>,
        usepeerassessment -> Nullable<Int2>,
        useselfassessment -> Nullable<Int2>,
        grade -> Nullable<Numeric>,
        gradinggrade -> Nullable<Numeric>,
        strategy -> Varchar,
        evaluation -> Varchar,
        gradedecimals -> Nullable<Int2>,
        nattachments -> Nullable<Int2>,
        latesubmissions -> Nullable<Int2>,
        maxbytes -> Nullable<Int8>,
        examplesmode -> Nullable<Int2>,
        submissionstart -> Nullable<Int8>,
        submissionend -> Nullable<Int8>,
        assessmentstart -> Nullable<Int8>,
        assessmentend -> Nullable<Int8>,
        phaseswitchassessment -> Int2,
        conclusion -> Nullable<Text>,
        conclusionformat -> Int2,
    }
}

table! {
    mdl_workshop_aggregations (id) {
        id -> Int8,
        workshopid -> Int8,
        userid -> Int8,
        gradinggrade -> Nullable<Numeric>,
        timegraded -> Nullable<Int8>,
    }
}

table! {
    mdl_workshopallocation_scheduled (id) {
        id -> Int8,
        workshopid -> Int8,
        enabled -> Int2,
        submissionend -> Int8,
        timeallocated -> Nullable<Int8>,
        settings -> Nullable<Text>,
        resultstatus -> Nullable<Int8>,
        resultmessage -> Nullable<Varchar>,
        resultlog -> Nullable<Text>,
    }
}

table! {
    mdl_workshop_assessments (id) {
        id -> Int8,
        submissionid -> Int8,
        reviewerid -> Int8,
        weight -> Int8,
        timecreated -> Nullable<Int8>,
        timemodified -> Nullable<Int8>,
        grade -> Nullable<Numeric>,
        gradinggrade -> Nullable<Numeric>,
        gradinggradeover -> Nullable<Numeric>,
        gradinggradeoverby -> Nullable<Int8>,
        feedbackauthor -> Nullable<Text>,
        feedbackauthorformat -> Nullable<Int2>,
        feedbackreviewer -> Nullable<Text>,
        feedbackreviewerformat -> Nullable<Int2>,
    }
}

table! {
    mdl_workshop_assessments_old (id) {
        id -> Int8,
        workshopid -> Int8,
        submissionid -> Int8,
        userid -> Int8,
        timecreated -> Int8,
        timegraded -> Int8,
        timeagreed -> Int8,
        grade -> Float8,
        gradinggrade -> Int2,
        teachergraded -> Int2,
        mailed -> Int2,
        resubmission -> Int2,
        donotuse -> Int2,
        generalcomment -> Nullable<Text>,
        teachercomment -> Nullable<Text>,
        newplugin -> Nullable<Varchar>,
        newid -> Nullable<Int8>,
    }
}

table! {
    mdl_workshop_comments_old (id) {
        id -> Int8,
        workshopid -> Int8,
        assessmentid -> Int8,
        userid -> Int8,
        timecreated -> Int8,
        mailed -> Int2,
        comments -> Text,
        newplugin -> Nullable<Varchar>,
        newid -> Nullable<Int8>,
    }
}

table! {
    mdl_workshop_elements_old (id) {
        id -> Int8,
        workshopid -> Int8,
        elementno -> Int2,
        description -> Text,
        scale -> Int2,
        maxscore -> Int2,
        weight -> Int2,
        stddev -> Float8,
        totalassessments -> Int8,
        newplugin -> Nullable<Varchar>,
        newid -> Nullable<Int8>,
    }
}

table! {
    mdl_workshopeval_best_settings (id) {
        id -> Int8,
        workshopid -> Int8,
        comparison -> Nullable<Int2>,
    }
}

table! {
    mdl_workshopform_accumulative (id) {
        id -> Int8,
        workshopid -> Int8,
        sort -> Nullable<Int8>,
        description -> Nullable<Text>,
        descriptionformat -> Nullable<Int2>,
        grade -> Int8,
        weight -> Nullable<Int4>,
    }
}

table! {
    mdl_workshopform_comments (id) {
        id -> Int8,
        workshopid -> Int8,
        sort -> Nullable<Int8>,
        description -> Nullable<Text>,
        descriptionformat -> Nullable<Int2>,
    }
}

table! {
    mdl_workshopform_numerrors (id) {
        id -> Int8,
        workshopid -> Int8,
        sort -> Nullable<Int8>,
        description -> Nullable<Text>,
        descriptionformat -> Nullable<Int2>,
        descriptiontrust -> Nullable<Int8>,
        grade0 -> Nullable<Varchar>,
        grade1 -> Nullable<Varchar>,
        weight -> Nullable<Int4>,
    }
}

table! {
    mdl_workshopform_numerrors_map (id) {
        id -> Int8,
        workshopid -> Int8,
        nonegative -> Int8,
        grade -> Numeric,
    }
}

table! {
    mdl_workshopform_rubric (id) {
        id -> Int8,
        workshopid -> Int8,
        sort -> Nullable<Int8>,
        description -> Nullable<Text>,
        descriptionformat -> Nullable<Int2>,
    }
}

table! {
    mdl_workshopform_rubric_config (id) {
        id -> Int8,
        workshopid -> Int8,
        layout -> Nullable<Varchar>,
    }
}

table! {
    mdl_workshopform_rubric_levels (id) {
        id -> Int8,
        dimensionid -> Int8,
        grade -> Numeric,
        definition -> Nullable<Text>,
        definitionformat -> Nullable<Int2>,
    }
}

table! {
    mdl_workshop_grades (id) {
        id -> Int8,
        assessmentid -> Int8,
        strategy -> Varchar,
        dimensionid -> Int8,
        grade -> Numeric,
        peercomment -> Nullable<Text>,
        peercommentformat -> Nullable<Int2>,
    }
}

table! {
    mdl_workshop_grades_old (id) {
        id -> Int8,
        workshopid -> Int8,
        assessmentid -> Int8,
        elementno -> Int8,
        feedback -> Text,
        grade -> Int2,
        newplugin -> Nullable<Varchar>,
        newid -> Nullable<Int8>,
    }
}

table! {
    mdl_workshop_old (id) {
        id -> Int8,
        course -> Int8,
        name -> Varchar,
        description -> Text,
        wtype -> Int2,
        nelements -> Int2,
        nattachments -> Int2,
        phase -> Int2,
        format -> Int2,
        gradingstrategy -> Int2,
        resubmit -> Int2,
        agreeassessments -> Int2,
        hidegrades -> Int2,
        anonymous -> Int2,
        includeself -> Int2,
        maxbytes -> Int8,
        submissionstart -> Int8,
        assessmentstart -> Int8,
        submissionend -> Int8,
        assessmentend -> Int8,
        releasegrades -> Int8,
        grade -> Int2,
        gradinggrade -> Int2,
        ntassessments -> Int2,
        assessmentcomps -> Int2,
        nsassessments -> Int2,
        overallocation -> Int2,
        timemodified -> Int8,
        teacherweight -> Int2,
        showleaguetable -> Int2,
        usepassword -> Int2,
        password -> Varchar,
        newplugin -> Nullable<Varchar>,
        newid -> Nullable<Int8>,
    }
}

table! {
    mdl_workshop_rubrics_old (id) {
        id -> Int8,
        workshopid -> Int8,
        elementno -> Int8,
        rubricno -> Int2,
        description -> Text,
        newplugin -> Nullable<Varchar>,
        newid -> Nullable<Int8>,
    }
}

table! {
    mdl_workshop_stockcomments_old (id) {
        id -> Int8,
        workshopid -> Int8,
        elementno -> Int8,
        comments -> Text,
        newplugin -> Nullable<Varchar>,
        newid -> Nullable<Int8>,
    }
}

table! {
    mdl_workshop_submissions (id) {
        id -> Int8,
        workshopid -> Int8,
        example -> Nullable<Int2>,
        authorid -> Int8,
        timecreated -> Int8,
        timemodified -> Int8,
        title -> Varchar,
        content -> Nullable<Text>,
        contentformat -> Int2,
        contenttrust -> Int2,
        attachment -> Nullable<Int2>,
        grade -> Nullable<Numeric>,
        gradeover -> Nullable<Numeric>,
        gradeoverby -> Nullable<Int8>,
        feedbackauthor -> Nullable<Text>,
        feedbackauthorformat -> Nullable<Int2>,
        timegraded -> Nullable<Int8>,
        published -> Nullable<Int2>,
        late -> Int2,
    }
}

table! {
    mdl_workshop_submissions_old (id) {
        id -> Int8,
        workshopid -> Int8,
        userid -> Int8,
        title -> Varchar,
        timecreated -> Int8,
        mailed -> Int2,
        description -> Text,
        gradinggrade -> Int2,
        finalgrade -> Int2,
        late -> Int2,
        nassessments -> Int8,
        newplugin -> Nullable<Varchar>,
        newid -> Nullable<Int8>,
    }
}

allow_tables_to_appear_in_same_query!(
    mdl_assign,
    mdl_assignfeedback_comments,
    mdl_assignfeedback_file,
    mdl_assign_grades,
    //mdl_assignment,
    //mdl_assignment_submissions,
    mdl_assign_plugin_config,
    mdl_assign_submission,
    mdl_assignsubmission_file,
    mdl_assignsubmission_onlinetext,
    mdl_assign_user_mapping,
    //mdl_backup_controllers,
    //mdl_backup_courses,
    //mdl_backup_files_template,
    //mdl_backup_ids_template,
    //mdl_backup_logs,
    mdl_block,
    mdl_block_community,
    mdl_block_instances,
    mdl_block_positions,
    mdl_block_rss_client,
    //mdl_blog_association,
    //mdl_blog_external,
    //mdl_book,
    //mdl_book_chapters,
    mdl_cache_filters,
    mdl_cache_flags,
    mdl_cache_text,
    mdl_capabilities,
    mdl_chat,
    mdl_chat_messages,
    mdl_chat_messages_current,
    mdl_chat_users,
    //mdl_choice,
    //mdl_choice_answers,
    //mdl_choice_options,
    mdl_cohort,
    mdl_cohort_members,
    mdl_comments,
    mdl_config,
    mdl_config_log,
    mdl_config_plugins,
    mdl_context,
    mdl_context_temp,
    mdl_course,
    mdl_course_categories,
    //mdl_course_completion_aggr_methd,
    //mdl_course_completion_crit_compl,
    //mdl_course_completion_criteria,
    //mdl_course_completions,
    mdl_course_format_options,
    mdl_course_modules,
    mdl_course_modules_availability,
    mdl_course_modules_avail_fields,
    mdl_course_modules_completion,
    mdl_course_published,
    mdl_course_request,
    mdl_course_sections,
    mdl_course_sections_availability,
    mdl_course_sections_avail_fields,
    //mdl_data,
    //mdl_data_content,
    //mdl_data_fields,
    //mdl_data_records,
    mdl_enrol,
    mdl_enrol_authorize,
    mdl_enrol_authorize_refunds,
    mdl_enrol_flatfile,
    mdl_enrol_paypal,
    mdl_event,
    mdl_events_handlers,
    mdl_events_queue,
    mdl_events_queue_handlers,
    mdl_event_subscriptions,
    mdl_external_functions,
    mdl_external_services,
    mdl_external_services_functions,
    mdl_external_services_users,
    mdl_external_tokens,
    mdl_feedback,
    mdl_feedback_completed,
    mdl_feedback_completedtmp,
    mdl_feedback_item,
    mdl_feedback_sitecourse_map,
    mdl_feedback_template,
    mdl_feedback_tracking,
    mdl_feedback_value,
    mdl_feedback_valuetmp,
    mdl_files,
    mdl_files_reference,
    mdl_filter_active,
    mdl_filter_config,
    mdl_folder,
    mdl_forum,
    mdl_forum_discussions,
    mdl_forum_posts,
    mdl_forum_queue,
    mdl_forum_read,
    mdl_forum_subscriptions,
    mdl_forum_track_prefs,
    mdl_glossary,
    mdl_glossary_alias,
    mdl_glossary_categories,
    mdl_glossary_entries,
    mdl_glossary_entries_categories,
    mdl_glossary_formats,
    mdl_grade_categories,
    mdl_grade_categories_history,
    mdl_grade_grades,
    mdl_grade_grades_history,
    mdl_grade_import_newitem,
    mdl_grade_import_values,
    mdl_grade_items,
    mdl_grade_items_history,
    mdl_grade_letters,
    mdl_grade_outcomes,
    mdl_grade_outcomes_courses,
    mdl_grade_outcomes_history,
    mdl_grade_settings,
    mdl_grading_areas,
    mdl_grading_definitions,
    mdl_gradingform_guide_comments,
    mdl_gradingform_guide_criteria,
    mdl_gradingform_guide_fillings,
    mdl_gradingform_rubric_criteria,
    mdl_gradingform_rubric_fillings,
    mdl_gradingform_rubric_levels,
    mdl_grading_instances,
    mdl_groupings,
    mdl_groupings_groups,
    mdl_groups,
    mdl_groups_members,
    mdl_imscp,
    mdl_label,
    mdl_lesson,
    mdl_lesson_answers,
    mdl_lesson_attempts,
    mdl_lesson_branch,
    mdl_lesson_grades,
    mdl_lesson_high_scores,
    mdl_lesson_pages,
    mdl_lesson_timer,
    mdl_license,
    mdl_log,
    mdl_log_display,
    mdl_log_queries,
    mdl_lti,
    mdl_lti_submission,
    mdl_lti_types,
    mdl_lti_types_config,
    mdl_message,
    mdl_message_contacts,
    mdl_message_processors,
    mdl_message_providers,
    mdl_message_read,
    mdl_message_working,
    mdl_mnet_application,
    mdl_mnet_host,
    mdl_mnet_host2service,
    mdl_mnet_log,
    mdl_mnet_remote_rpc,
    mdl_mnet_remote_service2rpc,
    mdl_mnet_rpc,
    mdl_mnet_service,
    mdl_mnet_service2rpc,
    mdl_mnetservice_enrol_courses,
    mdl_mnetservice_enrol_enrolments,
    mdl_mnet_session,
    mdl_mnet_sso_access_control,
    mdl_modules,
    mdl_my_pages,
    mdl_page,
    mdl_plagiarism_programming,
    mdl_plagiarism_programming_cours,
    mdl_plagiarism_programming_date,
    mdl_plagiarism_programming_jplag,
    mdl_plagiarism_programming_moss,
    mdl_plagiarism_programming_reslt,
    mdl_plagiarism_programming_rpt,
    mdl_plagiarism_turnitin_config,
    mdl_plagiarism_turnitin_files,
    mdl_portfolio_instance,
    mdl_portfolio_instance_config,
    mdl_portfolio_instance_user,
    mdl_portfolio_log,
    mdl_portfolio_mahara_queue,
    mdl_portfolio_tempdata,
    mdl_post,
    mdl_profiling,
    mdl_qtype_essay_options,
    //mdl_question,
    //mdl_question_answers,
    //mdl_question_attempts,
    //mdl_question_attempt_step_data,
    //mdl_question_attempt_steps,
    //mdl_question_calculated,
    //mdl_question_calculated_options,
    //mdl_question_categories,
    //mdl_question_dataset_definitions,
    //mdl_question_dataset_items,
    //mdl_question_datasets,
    //mdl_question_hints,
    //mdl_question_match,
    //mdl_question_match_sub,
    //mdl_question_multianswer,
    //mdl_question_multichoice,
    //mdl_question_numerical,
    //mdl_question_numerical_options,
    //mdl_question_numerical_units,
    //mdl_question_randomsamatch,
    //mdl_question_sessions,
    //mdl_question_shortanswer,
    //mdl_question_states,
    //mdl_question_truefalse,
    //mdl_question_usages,
    //mdl_quiz,
    //mdl_quiz_attempts,
    //mdl_quiz_feedback,
    //mdl_quiz_grades,
    //mdl_quiz_overrides,
    //mdl_quiz_overview_regrades,
    //mdl_quiz_question_instances,
    //mdl_quiz_question_response_stats,
    //mdl_quiz_question_statistics,
    mdl_quiz_reports,
    //mdl_quiz_statistics,
    mdl_rating,
    mdl_registration_hubs,
    mdl_repository,
    mdl_repository_instance_config,
    mdl_repository_instances,
    mdl_resource,
    mdl_resource_old,
    mdl_role,
    mdl_role_allow_assign,
    mdl_role_allow_override,
    mdl_role_allow_switch,
    mdl_role_assignments,
    mdl_role_capabilities,
    mdl_role_context_levels,
    mdl_role_names,
    mdl_role_sortorder,
    mdl_scale,
    mdl_scale_history,
    mdl_scorm,
    mdl_scorm_aicc_session,
    mdl_scorm_scoes,
    mdl_scorm_scoes_data,
    mdl_scorm_scoes_track,
    mdl_scorm_seq_mapinfo,
    mdl_scorm_seq_objective,
    mdl_scorm_seq_rolluprule,
    mdl_scorm_seq_rolluprulecond,
    mdl_scorm_seq_rulecond,
    mdl_scorm_seq_ruleconds,
    mdl_sessions,
    mdl_stats_daily,
    mdl_stats_monthly,
    mdl_stats_user_daily,
    mdl_stats_user_monthly,
    mdl_stats_user_weekly,
    mdl_stats_weekly,
    mdl_survey,
    mdl_survey_analysis,
    mdl_survey_answers,
    mdl_survey_questions,
    mdl_tag,
    mdl_tag_correlation,
    mdl_tag_instance,
    mdl_temp_enroled_template,
    mdl_temp_log_template,
    mdl_timezone,
    mdl_tool_customlang,
    mdl_tool_customlang_components,
    mdl_turnitintooltwo,
    mdl_turnitintooltwo_courses,
    mdl_turnitintooltwo_parts,
    mdl_turnitintooltwo_peermarks,
    mdl_turnitintooltwo_submissions,
    mdl_turnitintooltwo_users,
    mdl_upgrade_log,
    mdl_url,
    mdl_user,
    mdl_user_enrolments,
    mdl_user_info_category,
    mdl_user_info_data,
    mdl_user_info_field,
    mdl_user_lastaccess,
    mdl_user_preferences,
    mdl_user_private_key,
    mdl_webdav_locks,
    mdl_wiki,
    mdl_wiki_links,
    mdl_wiki_locks,
    mdl_wiki_pages,
    mdl_wiki_subwikis,
    mdl_wiki_synonyms,
    mdl_wiki_versions,
    mdl_workshop,
    mdl_workshop_aggregations,
    mdl_workshopallocation_scheduled,
    mdl_workshop_assessments,
    mdl_workshop_assessments_old,
    mdl_workshop_comments_old,
    mdl_workshop_elements_old,
    mdl_workshopeval_best_settings,
    mdl_workshopform_accumulative,
    mdl_workshopform_comments,
    mdl_workshopform_numerrors,
    mdl_workshopform_numerrors_map,
    mdl_workshopform_rubric,
    mdl_workshopform_rubric_config,
    mdl_workshopform_rubric_levels,
    mdl_workshop_grades,
    mdl_workshop_grades_old,
    mdl_workshop_old,
    mdl_workshop_rubrics_old,
    mdl_workshop_stockcomments_old,
    mdl_workshop_submissions,
    mdl_workshop_submissions_old,
);
