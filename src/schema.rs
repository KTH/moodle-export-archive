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

/* Empty
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

/ Empty
table! {
    mdl_cohort_members (id) {
        id -> Int8,
        cohortid -> Int8,
        userid -> Int8,
        timeadded -> Int8,
    }
}
 */

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

// File areas "submission_files", "feedback_files", and "feedback_files_batch".
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

/* Empty
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
/ Empty
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
/ Empty
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
/ Empty
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
/ Empty
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
/ Empty
table! {
    mdl_lesson_high_scores (id) {
        id -> Int8,
        lessonid -> Int8,
        userid -> Int8,
        gradeid -> Int8,
        nickname -> Varchar,
    }
}
/ Empty
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
/ Empty
table! {
    mdl_lesson_timer (id) {
        id -> Int8,
        lessonid -> Int8,
        userid -> Int8,
        starttime -> Int8,
        lessontime -> Int8,
    }
}
*/
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

/* Empty
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

/ Empty
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

/ Empty
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

/ Empty
table! {
    mdl_scorm_scoes_data (id) {
        id -> Int8,
        scoid -> Int8,
        name -> Varchar,
        value -> Text,
    }
}

/ Empty
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

/ Empty
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

/ Empty
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

/ Empty
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

/ Empty
table! {
    mdl_scorm_seq_rolluprulecond (id) {
        id -> Int8,
        scoid -> Int8,
        rollupruleid -> Int8,
        operator -> Varchar,
        cond -> Varchar,
    }
}

/ Empty
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

/ Empty
table! {
    mdl_scorm_seq_ruleconds (id) {
        id -> Int8,
        scoid -> Int8,
        conditioncombination -> Varchar,
        ruletype -> Int2,
        action -> Varchar,
    }
}
 */

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
/*
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
*/
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

allow_tables_to_appear_in_same_query!(
    mdl_assign,
    mdl_cache_filters,
    mdl_cache_flags,
    mdl_cache_text,
    mdl_capabilities,
    mdl_chat,
    mdl_chat_messages,
    mdl_chat_messages_current,
    mdl_chat_users,
    mdl_comments,
    mdl_config,
    mdl_config_log,
    mdl_config_plugins,
    mdl_context,
    mdl_context_temp,
    mdl_course,
    mdl_course_categories,
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
    mdl_groupings,
    mdl_groupings_groups,
    mdl_groups,
    mdl_groups_members,
    mdl_imscp,
    mdl_label,
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
    mdl_turnitintooltwo_courses,
    mdl_tool_customlang,
    mdl_tool_customlang_components,
    mdl_upgrade_log,
    mdl_url,
    /*
    mdl_user,
    mdl_user_enrolments,
    mdl_user_info_category,
    mdl_user_info_data,
    mdl_user_info_field,
    mdl_user_lastaccess,
    mdl_user_preferences,
    mdl_user_private_key,
     */
    mdl_webdav_locks,
    mdl_wiki,
    mdl_wiki_links,
    mdl_wiki_locks,
    mdl_wiki_pages,
    mdl_wiki_subwikis,
    mdl_wiki_synonyms,
    mdl_wiki_versions,
);
