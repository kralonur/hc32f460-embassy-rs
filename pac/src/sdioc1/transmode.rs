#[doc = "Register `TRANSMODE` reader"]
pub type R = crate::R<TransmodeSpec>;
#[doc = "Register `TRANSMODE` writer"]
pub type W = crate::W<TransmodeSpec>;
#[doc = "Field `BCE` reader - desc BCE"]
pub type BceR = crate::BitReader;
#[doc = "Field `BCE` writer - desc BCE"]
pub type BceW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ATCEN` reader - desc ATCEN"]
pub type AtcenR = crate::FieldReader;
#[doc = "Field `ATCEN` writer - desc ATCEN"]
pub type AtcenW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `DDIR` reader - desc DDIR"]
pub type DdirR = crate::BitReader;
#[doc = "Field `DDIR` writer - desc DDIR"]
pub type DdirW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `MULB` reader - desc MULB"]
pub type MulbR = crate::BitReader;
#[doc = "Field `MULB` writer - desc MULB"]
pub type MulbW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 1 - desc BCE"]
    #[inline(always)]
    pub fn bce(&self) -> BceR {
        BceR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bits 2:3 - desc ATCEN"]
    #[inline(always)]
    pub fn atcen(&self) -> AtcenR {
        AtcenR::new(((self.bits >> 2) & 3) as u8)
    }
    #[doc = "Bit 4 - desc DDIR"]
    #[inline(always)]
    pub fn ddir(&self) -> DdirR {
        DdirR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - desc MULB"]
    #[inline(always)]
    pub fn mulb(&self) -> MulbR {
        MulbR::new(((self.bits >> 5) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 1 - desc BCE"]
    #[inline(always)]
    pub fn bce(&mut self) -> BceW<'_, TransmodeSpec> {
        BceW::new(self, 1)
    }
    #[doc = "Bits 2:3 - desc ATCEN"]
    #[inline(always)]
    pub fn atcen(&mut self) -> AtcenW<'_, TransmodeSpec> {
        AtcenW::new(self, 2)
    }
    #[doc = "Bit 4 - desc DDIR"]
    #[inline(always)]
    pub fn ddir(&mut self) -> DdirW<'_, TransmodeSpec> {
        DdirW::new(self, 4)
    }
    #[doc = "Bit 5 - desc MULB"]
    #[inline(always)]
    pub fn mulb(&mut self) -> MulbW<'_, TransmodeSpec> {
        MulbW::new(self, 5)
    }
}
#[doc = "desc TRANSMODE\n\nYou can [`read`](crate::Reg::read) this register and get [`transmode::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`transmode::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct TransmodeSpec;
impl crate::RegisterSpec for TransmodeSpec {
    type Ux = u16;
}
#[doc = "`read()` method returns [`transmode::R`](R) reader structure"]
impl crate::Readable for TransmodeSpec {}
#[doc = "`write(|w| ..)` method takes [`transmode::W`](W) writer structure"]
impl crate::Writable for TransmodeSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets TRANSMODE to value 0"]
impl crate::Resettable for TransmodeSpec {}
