import {
  alpha,
  Box,
  BoxProps,
  Theme,
  Typography,
  useTheme,
} from "@mui/material";
import { BaseAsset, Checkbox } from "@/components/Atoms";
import { TOKENS } from "@/defi/Tokens";

const defaultFlexBoxProps = {
  display: "flex",
  alignItems: "center",
  justifyContent: "space-between",
  gap: 1,
};

const containerProps = (theme: Theme, selected?: boolean) =>
  ({
    py: 1.75,
    pl: 2,
    pr: 3,
    borderRadius: 9999,
    height: 56,
    sx: {
      background: selected
        ? alpha(theme.palette.primary.main, theme.custom.opacity.light)
        : undefined,
      border: `1px solid ${theme.palette.primary.main}`,
    },
    ...defaultFlexBoxProps,
  } as const);

export type CheckableXPabloItemBoxProps = {
  xPablo: any;
  selectedXPabloId?: number;
  setSelectedXPabloId?: (id?: number) => void;
} & BoxProps;

export const CheckableXPabloItemBox: React.FC<CheckableXPabloItemBoxProps> = ({
  xPablo,
  selectedXPabloId,
  setSelectedXPabloId,
  ...boxProps
}) => {
  const theme = useTheme();

  const selected = xPablo.id === selectedXPabloId;

  const handleChange = (event: React.ChangeEvent<HTMLInputElement>) => {
    setSelectedXPabloId?.(event.target.checked ? xPablo.id : undefined);
  };

  return (
    <Box {...containerProps(theme, selected)} {...boxProps}>
      <Box {...defaultFlexBoxProps}>
        <Checkbox
          value={xPablo.id}
          checked={selected}
          onChange={handleChange}
          inputProps={{ "aria-label": "controlled" }}
        />
        <BaseAsset icon={TOKENS.pablo.icon} label={`fNFT ${xPablo.id}`} />
      </Box>
      <Typography variant="body1">
        {`${xPablo.amount.toFormat()}(~$${xPablo.locked.toFormat()})`}
      </Typography>
    </Box>
  );
};
