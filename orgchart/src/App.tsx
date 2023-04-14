// @ts-nocheck
// TODO - Fix types

// https://codesandbox.io/s/corporate-hierarchy-1pbs0s used as a starting point.

import React, { useCallback, useMemo } from "react";
import { Tree, TreeNode, TreeProps } from "react-organizational-chart";
import _ from "lodash";
import clsx from "clsx";
import Card from "@mui/material/Card";
import CardContent from "@mui/material/CardContent";
import CardHeader from "@mui/material/CardHeader";
import Typography from "@mui/material/Typography";
import Box from "@mui/material/Box";
import IconButton from "@mui/material/IconButton";
import Person from "@mui/icons-material/Person4";
import Person3 from "@mui/icons-material/Person3Outlined";
import MoreVertIcon from "@mui/icons-material/MoreVert";
import Avatar from "@mui/material/Avatar";
import Menu from "@mui/material/Menu";
import MenuItem from "@mui/material/MenuItem";
import ExpandMoreIcon from "@mui/icons-material/ExpandMore";
import ListItemIcon from "@mui/material/ListItemIcon";
import ListItemText from "@mui/material/ListItemText";
import Badge from "@mui/material/Badge";
import Tooltip from "@mui/material/Tooltip";
import { DndProvider } from "react-dnd";
import { HTML5Backend } from "react-dnd-html5-backend";
import { useDrag, useDrop } from "react-dnd";

import { useQuery, gql } from "@apollo/client";

const GET_PEOPLE = gql`
  query {
    allOrgTiers {
      id
      nameEn
      tierLevel
      owner {
        givenName
        familyName
        activeRoles {
          titleEnglish
        }
      }
      parentOrganizationTier {
        id
      }
    }
  }
`;

const useStyles = {
  root: {
    background: "white",
    display: "inline-block",
    borderRadius: 16,
  },
  expand: {
    transform: "rotate(0deg)",
    marginTop: -10,
    marginLeft: "auto",
    //   transition: theme.transitions.create("transform", {
    //     duration: theme.transitions.duration.short
    //   })
    // },
  },
  expandOpen: {
    transform: "rotate(180deg)",
  },
  avatar: {
    backgroundColor: "#ECECF4",
  },
};

function Organization({ org, onCollapse, collapsed }: any) {
  const classes = useStyles;
  const [anchorEl, setAnchorEl] = React.useState(null);
  const elem = React.useRef(null);
  const [highlight, setHighlight] = React.useState(false);
  const handleOnCollapse = useCallback(() => {
    onCollapse(elem);
    setHighlight(true);
    setTimeout(() => {
      setHighlight(false);
    }, 1000);
  }, [onCollapse]);
  const handleClick = (event: {
    currentTarget: React.SetStateAction<null>;
  }) => {
    setAnchorEl(event.currentTarget);
  };
  const handleClose = () => {
    setAnchorEl(null);
  };
  const backgroundColor = highlight ? "#ECECF4" : "white";
  return (
    <Card
      variant="outlined"
      style={{ ...classes.root, backgroundColor }}
      ref={elem}
    >
      <CardHeader
        avatar={
          <Tooltip
            title={`${_.size(
              org.organizationChildRelationship
            )} managers, ${_.size(org.account)} others.`}
            arrow
          >
            <Badge
              style={{ cursor: "pointer" }}
              color="secondary"
              anchorOrigin={{
                vertical: "bottom",
                horizontal: "right",
              }}
              showZero
              invisible={!collapsed}
              overlap="circular"
              badgeContent={
                _.size(org.organizationChildRelationship) + _.size(org.account)
              }
              onClick={handleOnCollapse}
            >
              <Avatar style={classes.avatar}>
                <Person color="primary" />
              </Avatar>
            </Badge>
          </Tooltip>
        }
        title={org.tradingName}
        subheader={org.nameEn}
        action={
          <IconButton size="small" onClick={handleClick}>
            <MoreVertIcon />
          </IconButton>
        }
      />

      <Menu open={Boolean(anchorEl)} anchorEl={anchorEl} onClose={handleClose}>
        <MenuItem onClick={handleClose}>
          <ListItemIcon>
            <Person color="primary" />
          </ListItemIcon>
          <ListItemText primary="This menu does something cool." />
        </MenuItem>
      </Menu>
      <IconButton
        size="small"
        onClick={handleOnCollapse}
        style={{ ...classes.expand, ...(!collapsed ? classes.expandOpen : {}) }}
      >
        <ExpandMoreIcon />
      </IconButton>
    </Card>
  );
}
function Account({ a }: any) {
  const classes = useStyles;
  return (
    <Card
      variant="outlined"
      style={{ ...classes.root, cursor: "pointer" }}
    >
      <CardHeader
        avatar={
          <Avatar style={{ ...classes.avatar }}>
            <Person3 color="secondary" />
          </Avatar>
        }
        title={a.tradingName}
        subheader={a.nameEn}
      />
    </Card>
  );
}
function Node({ o, parent }) {
  const [collapsed, setCollapsed] = React.useState(o.collapsed);
  const handleCollapse = (elem) => {
    setCollapsed(!collapsed);
    setTimeout(() => {
      window.requestAnimationFrame(() => {
        const { x, y, width, height } = elem.current.getBoundingClientRect();
        window.scrollTo({
          top: window.scrollY + y - height / 2,
          left: window.scrollX + x + width / 2 - window.innerWidth / 2,
          behavior: "smooth",
        });
      });
    }, 0);
  };
  React.useEffect(() => {
    o.collapsed = collapsed;
  });
  const T = React.useMemo(
    () =>
      parent
        ? TreeNode
        : (props: JSX.IntrinsicAttributes & TreeProps) => (
            <Tree
              {...props}
              lineWidth={"2px"}
              lineColor={"#bbc"}
              lineBorderRadius={"12px"}
            >
              {props.children}
            </Tree>
          ),
    [parent]
  );
  const childNodes = !collapsed ? (
    <>
      {_.map(o.account, (a, idx) => (
        <TreeNode label={<Account a={a} />} key={idx} />
      ))}
      {_.map(o.organizationChildRelationship, (c, idx) => (
        <Node key={idx} o={c} parent={o} />
      ))}
    </>
  ) : undefined;
  return (
    <T
      label={
        <Organization
          org={o}
          onCollapse={handleCollapse}
          collapsed={collapsed}
        />
      }
    >
      {childNodes}
    </T>
  );
}
export default function App(props: any) {
  const { loading, error, data } = useQuery(GET_PEOPLE);

  const org = useMemo(() => {
    const d: any = {};
    let rootNode: any = {};
    if (loading || error) return null;
    data.allOrgTiers.forEach((r: any) => {
      const obj = {
        ...r,
        tradingName: `${r.owner.givenName} ${r.owner.familyName}`,
        account: [],
        organizationChildRelationship: [],
        collapsed: true,
      };
      d[r.id] = obj;
      if (r.parentOrganizationTier === null) rootNode = obj;
    });
    rootNode.collapsed = false;
    Object.keys(d).forEach((k) => {
      const obj = d[k];
      if (obj.parentOrganizationTier) {
        d[obj.parentOrganizationTier.id].organizationChildRelationship.push(
          obj
        );
      }
    });
    Object.keys(d).forEach((k) => {
      const obj = d[k];
      if (
        obj.parentOrganizationTier &&
        obj.organizationChildRelationship.length === 0
      ) {
        const parent = d[obj.parentOrganizationTier.id];
        parent.organizationChildRelationship.splice(
          parent.organizationChildRelationship.indexOf(obj),
          1
        );
        parent.account.push(obj);
      }
    });
    return rootNode;
  }, [loading, error, data]);

  if (loading) return <p>Loading data..</p>;
  if (error) return <p>Error: {error.message}</p>;
  return (
    <Box bgcolor="background" padding={4} height="80vh">
      <DndProvider backend={HTML5Backend}>
        <Node o={org} />
      </DndProvider>
    </Box>
  );
}
